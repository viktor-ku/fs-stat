import toml from '@iarna/toml';

interface PkgStruct {
  name: string
  version: string
}

interface CargoStruct {
  package: PkgStruct[]
}

export function readVersion(content: string): string {
  const parsed = toml.parse(content) as any as CargoStruct;
  return parsed.package
    .find(pkg => pkg.name === "fs-stat")!
    .version
}

export function writeVersion(content: string, version: string): string {
  const parsed = toml.parse(content) as any as CargoStruct;

  const next = parsed.package.map(pkg => {
    if (pkg.name === 'fs-stat') {
      return {
        ...pkg,
        version,
      }
    }

    return pkg
  })

  return toml.stringify({
    ...parsed,
    package: next as any,
  })
}

export function isPrivate(): boolean {
  return false
}
