import * as v from "valibot";
import { FieldKind } from "./types";

export const NumberSchema = v.pipe(v.string(), v.transform(Number), v.number());
export const TextSchema = v.string();
export const DateSchema = v.pipe(
	v.string(),
	v.transform((input) => new Date(input)),
	v.date(),
);

export const parseValidationSchema = {
	[FieldKind.TEXT]: TextSchema,
	[FieldKind.NUMBER]: NumberSchema,
	[FieldKind.DATE]: DateSchema,
};

const isValidFieldKind = (
	expectedType: FieldKind,
)  => {
	return v.is(v.enum(FieldKind), expectedType);
};

export const validInput = (expectedType: FieldKind, receivedValue: unknown) => {
  console.log(expectedType, receivedValue);
  if (!isValidFieldKind(expectedType)) throw Error();
	return v.parse(parseValidationSchema[expectedType], receivedValue);
};
