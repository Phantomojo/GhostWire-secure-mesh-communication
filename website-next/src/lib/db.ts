import { getCloudflareContext } from "@opennextjs/cloudflare";

export interface D1Database {
  prepare(query: string): D1PreparedStatement;
  exec(query: string): Promise<{ success: boolean }>;
  batch<T>(statements: D1PreparedStatement[]): Promise<T[]>;
}

export interface D1PreparedStatement {
  bind(...values: unknown[]): D1PreparedStatement;
  run(): Promise<{ success: boolean; meta: { last_row_id: number } }>;
  all<T = Record<string, unknown>>(): Promise<{ results: T[] }>;
  first<T = Record<string, unknown>>(): Promise<T | null>;
}

export function getDb(): D1Database {
  const { env } = getCloudflareContext();
  return (env as Record<string, unknown>).DB as D1Database;
}
