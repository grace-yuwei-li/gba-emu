import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const response = await resolve(event);
	response.headers.append('Cross-Origin-Opener-Policy', 'same-origin');
	response.headers.append('Cross-Origin-Embedder-Policy', 'require-corp');
	return response;
};
