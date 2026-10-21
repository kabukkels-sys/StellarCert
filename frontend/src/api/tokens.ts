import type { User } from './types';

/**
 * Callback invoked after `apiClient` silently refreshes the access token, so
 * AuthContext can keep its reactive `isAuthenticated` / `user` state in sync.
 * The fresh `user` from the refresh response is forwarded when available.
 */
type TokenRefreshCallback = (accessToken: string, user?: User | null) => void;

let _onTokenRefreshed: TokenRefreshCallback | null = null;
export const setTokenRefreshCallback = (cb: TokenRefreshCallback) => {
  _onTokenRefreshed = cb;
};
export const notifyTokenRefreshed = (accessToken: string, user?: User | null) => {
  _onTokenRefreshed?.(accessToken, user);
};

let _inMemoryAccessToken: string | null = null;

export const tokenStorage = {
  getAccessToken: (): string | null => _inMemoryAccessToken,
  setAccessToken: (token: string): void => {
    _inMemoryAccessToken = token;
  },
  clearTokens: (): void => {
    _inMemoryAccessToken = null;
  },
  hasAccessToken: (): boolean => !!_inMemoryAccessToken,
};

export type BootstrapResult = {
  accessToken: string | null;
  user?: User | null;
};

let _bootstrapPromise: Promise<BootstrapResult> | null = null;

/**
 * Performs exactly one explicit `/auth/refresh` during initial mount so that
 * protected requests can assume a valid token. Concurrent callers share the
 * same in-flight promise (de-duplication), and a successful result is cached
 * in `tokenStorage`. A failed refresh resolves to a null token rather than
 * throwing, so the app can render the unauthenticated shell.
 */
export const bootstrapAccessToken = (
  refresh: () => Promise<{ accessToken: string; user?: User | null }>,
): Promise<BootstrapResult> => {
  const existing = tokenStorage.getAccessToken();
  if (existing) {
    return Promise.resolve({ accessToken: existing });
  }
  if (_bootstrapPromise) {
    return _bootstrapPromise;
  }
  const promise = refresh()
    .then(({ accessToken, user }) => {
      tokenStorage.setAccessToken(accessToken);
      notifyTokenRefreshed(accessToken, user);
      return { accessToken, user };
    })
    .catch(() => ({ accessToken: null }))
    .finally(() => {
      if (_bootstrapPromise === promise) {
        _bootstrapPromise = null;
      }
    });
  _bootstrapPromise = promise;
  return promise;
};

/** Test-only reset for the bootstrap de-duplication guard. */
export const __resetBootstrapForTests = (): void => {
  _bootstrapPromise = null;
};
