export const dynamic = "force-dynamic";
export async function GET(request: Request) {
    const url = new URL(request.url);
    const pathname = url.pathname;
    return await fetch(`${process.env.API || ''}${pathname.slice(4)}${url.search}`, {
        method: request.method,
        signal: request.signal,
        body: request.body,
        headers: request.headers,
        cache: 'no-store',
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        //    @ts-expect-error
        duplex: "half",
    });
}
export async function POST(request: Request) {
  const url = new URL(request.url);
  const pathname = url.pathname;
  return await fetch(`${process.env.API || ''}${pathname.slice(4)}${url.search}`, {
      method: request.method,
      signal: request.signal,
      body: request.body,
      headers: request.headers,
      cache: 'no-store',
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      //    @ts-expect-error
      duplex: "half",
  });
}
export async function PUT(request: Request) {
  const url = new URL(request.url);
  const pathname = url.pathname;
  return await fetch(`${process.env.API || ''}${pathname.slice(4)}${url.search}`, {
      method: request.method,
      signal: request.signal,
      body: request.body,
      headers: request.headers,
      cache: 'no-store',
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      //    @ts-expect-error
      duplex: "half",
  });
}

export async function PATCH(request: Request) {
  const url = new URL(request.url);
  const pathname = url.pathname;
  return await fetch(`${process.env.API || ''}${pathname.slice(4)}${url.search}`, {
      method: request.method,
      signal: request.signal,
      body: request.body,
      headers: request.headers,
      cache: 'no-store',
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      //    @ts-expect-error
      duplex: "half",
  });
}

export async function DELETE(request: Request) {
  const url = new URL(request.url);
  const pathname = url.pathname;
  return await fetch(`${process.env.API || ''}${pathname.slice(4)}${url.search}`, {
      method: request.method,
      signal: request.signal,
      body: request.body,
      headers: request.headers,
      cache: 'no-store',
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      //    @ts-expect-error
      duplex: "half",
  });
}