import { client, type ModelDefinition } from "filigree-svelte";
import { z } from "zod";
import { ObjectPermission } from "../model_types.js";

export type TagId = string;

export const TagSchema = z.object({
	id: z.string(),
	organization_id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	name: z.string(),
	color: z.string(),
});

export type Tag = z.infer<typeof TagSchema>;
export const TagListResultSchema = TagSchema;
export type TagListResult = Tag;
export const TagPopulatedGetResultSchema = TagSchema;
export type TagPopulatedGetResult = Tag;
export const TagPopulatedListResultSchema = TagSchema;
export type TagPopulatedListResult = Tag;
export const TagCreateResultSchema = TagSchema;
export type TagCreateResult = Tag;

export const TagCreatePayloadAndUpdatePayloadSchema = z.object({
	id: z.string().optional(),
	name: z.string(),
	color: z.string(),
});

export type TagCreatePayloadAndUpdatePayload = z.infer<
	typeof TagCreatePayloadAndUpdatePayloadSchema
>;
export const TagCreatePayloadSchema = TagCreatePayloadAndUpdatePayloadSchema;
export type TagCreatePayload = TagCreatePayloadAndUpdatePayload;
export const TagUpdatePayloadSchema = TagCreatePayloadAndUpdatePayloadSchema;
export type TagUpdatePayload = TagCreatePayloadAndUpdatePayload;

export const baseUrl = "tags";
export const urlWithId = (id: string) => `${baseUrl}/${id}`;

export const urls = {
	create: baseUrl,
	list: baseUrl,
	get: urlWithId,
	update: urlWithId,
	delete: urlWithId,
};

export const TagModel: ModelDefinition<typeof TagSchema> = {
	name: "Tag",
	plural: "Tags",
	baseUrl,
	urls,
	schema: TagSchema,
	createSchema: TagCreatePayloadSchema,
	updateSchema: TagUpdatePayloadSchema,
	fields: [
		{
			name: "id",
			type: "uuid",
			label: "Id",
			constraints: {
				required: true,
			},
		},
		{
			name: "organization_id",
			type: "uuid",
			label: "Organization Id",
			constraints: {
				required: true,
			},
		},
		{
			name: "updated_at",
			type: "date-time",
			label: "Updated At",
			constraints: {
				required: true,
			},
		},
		{
			name: "created_at",
			type: "date-time",
			label: "Created At",
			constraints: {
				required: true,
			},
		},
		{
			name: "name",
			type: "text",
			label: "Name",
			constraints: {
				required: true,
			},
		},
		{
			name: "color",
			type: "text",
			label: "Color",
			constraints: {
				required: true,
			},
		},
	],
};
