import { client, type ModelDefinition } from "filigree-svelte";
import { z } from "zod";
import { ObjectPermission } from "../model_types.js";
import {
	CommentCreatePayloadSchema,
	CommentSchema,
	CommentUpdatePayloadSchema,
} from "./comment.js";
import {
	PollCreatePayloadSchema,
	PollSchema,
	PollUpdatePayloadSchema,
} from "./poll.js";
import {
	PostImageCreatePayloadSchema,
	PostImageSchema,
	PostImageUpdatePayloadSchema,
} from "./post_image.js";
import {
	ReactionCreatePayloadSchema,
	ReactionSchema,
	ReactionUpdatePayloadSchema,
} from "./reaction.js";
import {
	TagCreatePayloadSchema,
	TagSchema,
	TagUpdatePayloadSchema,
} from "./tag.js";

export type PostId = string;

export const PostSchema = z.object({
	id: z.string(),
	organization_id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	subject: z.string(),
	body: z.string(),
});

export type Post = z.infer<typeof PostSchema>;
export const PostListResultSchema = PostSchema;
export type PostListResult = Post;

export const PostCreatePayloadSchema = z.object({
	id: z.string().optional(),
	subject: z.string(),
	body: z.string(),
	tag: z.string().optional(),
});

export type PostCreatePayload = z.infer<typeof PostCreatePayloadSchema>;

export const PostCreateResultSchema = z.object({
	id: z.string(),
	organization_id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	subject: z.string(),
	body: z.string(),
	tag: z.string().optional(),
});

export type PostCreateResult = z.infer<typeof PostCreateResultSchema>;

export const PostPopulatedGetResultSchema = z.object({
	id: z.string(),
	organization_id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	subject: z.string(),
	body: z.string(),
	comment_ids: z.string().array(),
	reactions: ReactionSchema.array(),
	poll: PollSchema.optional(),
	tag: TagSchema.optional(),
	images: PostImageSchema.array(),
});

export type PostPopulatedGetResult = z.infer<
	typeof PostPopulatedGetResultSchema
>;

export const PostPopulatedListResultSchema = z.object({
	id: z.string(),
	organization_id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	subject: z.string(),
	body: z.string(),
	comment_ids: z.string().array(),
	poll_id: z.string().optional(),
	tag_id: z.string().optional(),
});

export type PostPopulatedListResult = z.infer<
	typeof PostPopulatedListResultSchema
>;

export const PostUpdatePayloadSchema = z.object({
	id: z.string().optional(),
	subject: z.string(),
	body: z.string(),
	tag: z.string().nullish().optional(),
});

export type PostUpdatePayload = z.infer<typeof PostUpdatePayloadSchema>;

export const baseUrl = "posts";
export const urlWithId = (id: string) => `${baseUrl}/${id}`;

export const urls = {
	create: baseUrl,
	list: baseUrl,
	get: urlWithId,
	update: urlWithId,
	delete: urlWithId,
};

export const PostModel: ModelDefinition<typeof PostSchema> = {
	name: "Post",
	plural: "Posts",
	baseUrl,
	urls,
	schema: PostSchema,
	createSchema: PostCreatePayloadSchema,
	updateSchema: PostUpdatePayloadSchema,
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
			name: "subject",
			type: "text",
			label: "Subject",
			description: "A summary of the post",
			constraints: {
				required: true,
			},
		},
		{
			name: "body",
			type: "text",
			label: "Body",
			description: "The text of the post",
			constraints: {
				required: true,
			},
		},
	],
};
