Deno.serve( {port: 6969}, async (req: Request) => {
  const url = new URL(req.url);
  const params = url.searchParams;

  const instance = params.get("instance");
  if (instance == undefined) return new Response("Instance cannot be empty");

  const owner = params.get("owner");
  if (instance == undefined) return new Response("Owner cannot be empty");

  const repo = params.get("repo");
  if (instance == undefined) return new Response("Repo cannot be empty");

  const link = await generate_badge(instance, owner, repo);

  console.log(instance, owner, repo);
  console.log("Link:", link);
  return Response.redirect(link)
})

async function generate_badge(instance: string, owner: string, repo: string): Promise<string> {
    const link = `https://${instance}/api/v1/repos/${owner}/${repo}/languages`;

    const resp = await fetch(link);
    const body = await resp.json();
    console.log("Body: ", body);
    
    const bytes = Object.values(body);
    console.log("Bytes: ", bytes)
    const total = bytes.reduce((a,b) => a+b, 0)
    console.log("Total: ",total)

    const [lang, bites] = Object.entries(body)[0];
    console.log(`Lang: ${lang}, Bytes: ${bites}`)

    const percentage = Math.floor((bites / total) * 100)

    return `https://img.shields.io/badge/${lang}-${percentage}%25-blue?logo=${lang}`
}

