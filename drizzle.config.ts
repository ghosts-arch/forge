import { defineConfig } from "drizzle-kit";

if (!process.env.DATABASE_URL) throw Error("DATABASE_URL is not set");

export default defineConfig({
	out: "./drizzle",
	schema: "./src/lib/server/database/schema.ts",
	dialect: "postgresql",
	dbCredentials: {
		url: process.env.DATABASE_URL,
	},
	driver: "pglite",
});
