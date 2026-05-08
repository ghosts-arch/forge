import { drizzle } from "drizzle-orm/pglite";
import { env } from "$env/dynamic/private";
import { relations } from "./relations";
import * as schema from "./schema";
if (!env.DATABASE_URL) throw Error("DATABASE_URL is not set");

export const db = drizzle(env.DATABASE_URL, { relations, schema });
