import { useTranslation } from "react-i18next";
import type { ContentType, TorrentInfo } from "../types/api";

interface Props {
  torrentInfo: TorrentInfo;
  onConfirm: (contentType: ContentType, torrentInfo: TorrentInfo) => void;
  onCancel: () => void;
}

export default function TorrentContentTypePicker({
  torrentInfo,
  onConfirm,
  onCancel,
}: Props) {
  const { t } = useTranslation();
  const options: { value: ContentType; label: string }[] = [
    { value: "film", label: t("common.film") },
    { value: "serie", label: t("common.serie") },
    { value: "jeu", label: t("common.jeu") },
  ];

  return (
    <div className="max-w-lg mx-auto p-6">
      <h2 className="text-xl font-semibold mb-2">{t("torrentPicker.title")}</h2>
      <p className="text-fg-muted text-sm mb-1">
        {t("torrentPicker.torrent")}<span className="text-fg-bright">{torrentInfo.meta.name}</span>
      </p>
      <p className="text-fg-muted text-sm mb-4">
        {t("torrentPicker.size")}{torrentInfo.size_formatted}
        {torrentInfo.parsed.title && (
          <> &middot; {t("torrentPicker.detectedTitle")}<span className="text-fg-bright">{torrentInfo.parsed.title}</span></>
        )}
      </p>

      <p className="text-sm text-fg mb-3">
        {t("torrentPicker.question")}
      </p>

      <div className="flex gap-3">
        {options.map((opt) => (
          <button
            key={opt.value}
            onClick={() => onConfirm(opt.value, torrentInfo)}
            className="flex-1 bg-input hover:bg-blue-600 border border-edge hover:border-blue-500 text-white px-4 py-3 rounded text-sm font-medium transition-colors"
          >
            {opt.label}
          </button>
        ))}
      </div>

      <button
        onClick={onCancel}
        className="mt-4 text-fg-muted hover:text-fg-bright text-sm transition-colors"
      >
        {t("common.cancel")}
      </button>
    </div>
  );
}
