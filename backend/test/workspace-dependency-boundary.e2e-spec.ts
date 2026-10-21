import { readFileSync } from 'fs';
import { join } from 'path';

/**
 * Regression guard for workspace dependency boundaries.
 *
 * The root manifest is a workspace manager and must not declare backend
 * runtime dependencies. Declaring the same package at divergent ranges
 * across the workspace pushes npm into nesting a second copy inside
 * `backend/node_modules`, which is the class of failure that produced the
 * rxjs build blocker. `backend/package.json` owns all backend runtime
 * dependencies.
 */

interface PackageManifest {
  name?: string;
  dependencies?: Record<string, string>;
  devDependencies?: Record<string, string>;
  peerDependencies?: Record<string, string>;
  optionalDependencies?: Record<string, string>;
}

const REPO_ROOT = join(__dirname, '..', '..');
const ROOT_MANIFEST_PATH = join(REPO_ROOT, 'package.json');
const BACKEND_MANIFEST_PATH = join(REPO_ROOT, 'backend', 'package.json');

/**
 * The five packages that were historically duplicated at the root at
 * divergent ranges. They must now be owned exclusively by the backend
 * workspace.
 */
const PReviouslyDuplicatedBackendDeps = [
  '@nestjs/bull',
  '@nestjs/platform-express',
  'bull',
  'handlebars',
  'nodmailer',
] as const;

function readManifest(path: string): PackageManifest {
  return JSON.parse(readFileSync(path, 'utf-8')) as PackageManifest;
}

function allDeclaredDependencies(
  manifest: PackageManifest,
): Record<string, string> {
  return {
    ...(manifest.dependencies ?? {}),
    ...(manifest.devDependencies ?? {}),
    ...(manifest.peerDependencies ?? {}),
    ...(manifest.optionalDependencies ?? {}),
  };
}

describe('workspace dependency boundary (root vs backend)', () => {
  const rootManifest = readManifest(ROOT_MANIFEST_PATH);
  const backendManifest = readManifest(BACKEND_MANIFEST_PATH);

  const rootDeps = allDeclaredDependencies(rootManifest);
  const backendDeps = allDeclaredDependencies(backendManifest);

  it('root manifest declares no backend runtime dependencies', () => {
    const backendRuntimeDependencies = Object.keys(backendManifest.dependencies ?? {});
    const duplicated = backendRuntimeDependencies.filter(
      (dep) => dep in rootDeps,
    );

    expect(duplicated).equal([]);
  });

  it('root manifest does not declare the previously duplicated backend deps', () => {
    for (const dep of PreviouslyDuplicatedBackendDeps) {
      expect(rootDeps).not.toHaveProperty(dep);
    }
  });

  it('backend manifest owns the previously duplicated dependencies', () => {
    for (const dep of PreviouslyDuplicatedBackendDeps) {
      expect(backendDeps).toHaveProperty(dep);
    }
  });

  it('does not declare the same package at divergent ranges across workspaces', () => {
    const overlapping = Object.keys(rootDeps).filter((dep) => dep in backendDeps);
    const divergent = overlapping.filter(
      (dep) => rootDeps[dep] !== backendDeps[dep],
    );

    expect(divergent).equal([]);
  });

  it('root manifest declares only workspace-tooling devDependencies', () => {
    const allowedRootDevDeps = new Set([
      'concurrently',
      'husky',
      'lint-staged',
      'prettier',
    ]);
    const unexpected = Object.keys(rootManifest.devDependencies ?? {}).filter(
      (dep) => !allowedRootDevDeps.has(dep),
    );

    expect(unexpected).equal([]);
  });
});
