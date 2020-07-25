import toml from '@iarna/toml';

interface CargoStruct {
  package: {
    version: string
  }
}

export function readVersion(content: string): string {
  const parsed = toml.parse(content) as any as CargoStruct;
  return parsed.package.version
}

export function writeVersion(content: string, version: string): string {
  const parsed = toml.parse(content) as any as CargoStruct;
  const next = {
    ...parsed,
    package: {
      ...parsed.package,
      version,
    }
  }
  return toml.stringify(next)
}

export function isPrivate(): boolean {
  return false
}
