import { useQuery } from"@tanstack/react-query";
import { PageHeader } from"@/components/PageHeader";
import { commands } from"@/lib/bindings";

export function SettingsPage() {
  const info = useQuery({
    queryKey: ["appInfo"],
    queryFn: async () => {
      const res = await commands.appInfo();
      if (res.status ==="error") throw new Error(res.error.message);
      return res.data;
    },
  });

  return (
    <div>
      <PageHeader title="Settings" subtitle="TorBox account and search sources arrive in milestones 2 and 3b." />
      <section className="rounded-lg border border-border p-4 text-sm">
        <h3 className="mb-2 font-medium">About</h3>
        {info.isPending && <p>Loading…</p>}
        {info.isError && <p className="text-destructive">Error: {info.error.message}</p>}
        {info.data && (
          <dl className="grid grid-cols-[max-content_1fr] gap-x-4 gap-y-1">
            <dt className="text-muted-foreground">Version</dt>
            <dd>{info.data.version}</dd>
            <dt className="text-muted-foreground">Database</dt>
            <dd>{info.data.dbOk ?"OK" :"Unavailable"}</dd>
          </dl>
        )}
      </section>
    </div>
  );
}
