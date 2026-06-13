import type { validInput } from "./validation";

export enum FieldKind {
	TEXT = "text",
	NUMBER = "number",
	DATE = "date",
}

export type AssetField = {
	uuid?: string;
	asset_id: string;
	name: string;
	kind: FieldKind;
	value: ReturnType<typeof validInput>;
};

export type Asset = {
	uuid: string;
	name: string;
	fields: AssetField[];
};

export type Relation = {
	uuid: string;
	description: string;
	source_asset_uuid: string;
	target_asset_uuid: string;
	name: string;
};

export type AssetInformations = Pick<Asset, "name" | "uuid">;
