import type { HandleClientError } from "@sveltejs/kit";

import * as Sentry from "@sentry/sveltekit";
import { PUBLIC_SENTRY_DSN } from "$env/static/public";

if (PUBLIC_SENTRY_DSN) {
	Sentry.init({
		dsn: PUBLIC_SENTRY_DSN,
		tracesSampleRate: 1.0,
	});
}

const errorHandler: HandleClientError = ({ error, event, message, status }) => {
	console.dir(error);
	return {
		status,
		message,
		error,
	};
};

export const handleError = Sentry.handleErrorWithSentry(errorHandler);
