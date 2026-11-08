import { usePlayerStore } from"@/stores/player";

export function PlayerBar() {
  const title = usePlayerStore((s) => s.title);
  return (
    <footer className="flex h-16 shrink-0 items-center border-t border-border px-4 text-sm text-muted-foreground">
      {title ??"Nothing playing"}
    </footer>
  );
}
