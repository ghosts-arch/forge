import type { HTMLInputTypeAttribute } from "svelte/elements";
import type { validInput } from "./validation";

export const FieldKind = ["text", "number", "date"] as const;

export type AssetField = {
	asset_id: string;
	name: string;
	kind: HTMLInputTypeAttribute;
	value: ReturnType<typeof validInput>;
};

export type Asset = {
	uuid: string;
	name: string;
	fields: AssetField[];
};
