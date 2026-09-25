type Election = { id: string; status: string; options: string[]; votes: string[] };
const output = document.querySelector<HTMLPreElement>('#state')!;
fetch(import.meta.env.VITE_API_URL ?? 'http://localhost:8080/api/v1/elections/demo')
  .then(r => r.json() as Promise<Election>).then(e => { output.textContent = JSON.stringify(e, null, 2); })
  .catch(() => { output.textContent = 'API no disponible. Inicia docker compose.'; });
