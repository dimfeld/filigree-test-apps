import { client, type ModelDefinition } from "filigree-svelte";
import { z } from "zod";
import { ObjectPermission } from "../model_types.js";

export const PostTagSchema = z.object({
	post_id: z.string(),
	tag_id: z.string(),
	organization_id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
});

export type PostTag = z.infer<typeof PostTagSchema>;
export const PostTagListResultSchema = PostTagSchema;
export type PostTagListResult = PostTag;
export const PostTagPopulatedGetResultSchema = PostTagSchema;
export type PostTagPopulatedGetResult = PostTag;
export const PostTagPopulatedListResultSchema = PostTagSchema;
export type PostTagPopulatedListResult = PostTag;
export const PostTagCreateResultSchema = PostTagSchema;
export type PostTagCreateResult = PostTag;

export const PostTagCreatePayloadAndUpdatePayloadSchema = z.object({
	post_id: z.string().optional(),
	tag_id: z.string().optional(),
});

export type PostTagCreatePayloadAndUpdatePayload = z.infer<
	typeof PostTagCreatePayloadAndUpdatePayloadSchema
>;
export const PostTagCreatePayloadSchema =
	PostTagCreatePayloadAndUpdatePayloadSchema;
export type PostTagCreatePayload = PostTagCreatePayloadAndUpdatePayload;
export const PostTagUpdatePayloadSchema =
	PostTagCreatePayloadAndUpdatePayloadSchema;
export type PostTagUpdatePayload = PostTagCreatePayloadAndUpdatePayload;

export const baseUrl = "post_tags";
export const urlWithId = (id: string) => `${baseUrl}/${id}`;

export const urls = {
	create: baseUrl,
	list: baseUrl,
	get: urlWithId,
	update: urlWithId,
	delete: urlWithId,
};

export const PostTagModel: ModelDefinition<typeof PostTagSchema> = {
	name: "PostTag",
	plural: "PostTags",
	baseUrl,
	urls,
	schema: PostTagSchema,
	createSchema: PostTagCreatePayloadSchema,
	updateSchema: PostTagUpdatePayloadSchema,
	fields: [
		{
			name: "post_id",
			type: "uuid",
			label: "Post Id",
			constraints: {
				required: true,
			},
		},
		{
			name: "tag_id",
			type: "uuid",
			label: "Tag Id",
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
	],
};
