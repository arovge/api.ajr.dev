import { Env } from ".";

export default {
    isAuthorized: (request: Request, env: Env): boolean => request.headers.get('X-API-Key') === env.SHARED_API_KEY,
    ok: (): Promise<Response> => Promise.resolve(new Response('OK', { status: 200 })),
    unauthorized: (): Promise<Response> => Promise.resolve(new Response('Unauthorized', { status: 401 })),
    forbidden: (): Promise<Response> => Promise.resolve(new Response('Forbidden', { status: 403 })),
    notFound: (): Promise<Response> => Promise.resolve(new Response('404s and heartbreaks', { status: 404 })),
    methodNotAllowed: (allowedMethods: string[]): Response => {
        const headers = allowedMethods.length === 0 ? undefined : {
            Allow: allowedMethods.join(', ')
        };
        return new Response('Method Not Allowed', { status: 405, headers });
    }
}
