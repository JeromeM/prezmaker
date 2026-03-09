import { open } from "@tauri-apps/plugin-dialog";

interface Props {
  onImport: (filePath: string) => void;
  disabled?: boolean;
}

export default function TorrentImport({ onImport, disabled }: Props) {
  const handleClick = async () => {
    const path = await open({
      filters: [{ name: "Torrent", extensions: ["torrent"] }],
      multiple: false,
    });
    if (path) {
      onImport(path as string);
    }
  };

  return (
    <button
      type="button"
      onClick={handleClick}
      disabled={disabled}
      className="text-gray-400 hover:text-white disabled:text-gray-600 transition-colors p-2"
      title="Importer un .torrent"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
        className="w-5 h-5"
      >
        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
        <polyline points="7 10 12 15 17 10" />
        <line x1="12" y1="15" x2="12" y2="3" />
      </svg>
    </button>
  );
}
