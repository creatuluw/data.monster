import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

export const POST: RequestHandler = async ({ request }) => {
	const { messages } = await request.json();

	const apiUrl = env.LLM_API_URL || 'https://api.z.ai/api/coding/paas/v4/chat/completions';
	const apiKey = env.LLM_API_KEY || '';
	const model = env.LLM_MODEL || 'glm-5';

	const response = await fetch(apiUrl, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${apiKey}`
		},
		body: JSON.stringify({
			model,
			messages,
			stream: true,
			temperature: 0.6
		})
	});

	if (!response.ok) {
		const errorText = await response.text();
		return new Response(errorText, {
			status: response.status,
			headers: { 'Content-Type': 'application/json' }
		});
	}

	return new Response(response.body, {
		headers: {
			'Content-Type': 'text/event-stream',
			'Cache-Control': 'no-cache',
			Connection: 'keep-alive'
		}
	});
};
