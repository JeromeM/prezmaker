import { useState, useMemo } from "react";
import { useTranslation } from "react-i18next";

interface TagDef {
  label: string;
  open: string;
  close?: string;
  placeholder?: string;
}

interface Category {
  name: string;
  tags: TagDef[];
}

interface Props {
  collapsed: boolean;
  onToggle: () => void;
  onInsertTag: (open: string, close?: string, placeholder?: string) => void;
}

export default function BBCodePalette({ collapsed, onToggle, onInsertTag }: Props) {
  const { t } = useTranslation();
  const [openCats, setOpenCats] = useState<Set<string>>(new Set(["text"]));

  const categories: Category[] = useMemo(
    () => [
      {
        name: t("bbcodePalette.text"),
        tags: [
          { label: t("bbcodePalette.bold"), open: "[b]", close: "[/b]", placeholder: "texte" },
          { label: t("bbcodePalette.italic"), open: "[i]", close: "[/i]", placeholder: "texte" },
          { label: t("bbcodePalette.underline"), open: "[u]", close: "[/u]", placeholder: "texte" },
          { label: t("bbcodePalette.color"), open: '[color=#FF0000]', close: "[/color]", placeholder: "texte" },
          { label: t("bbcodePalette.fontSize"), open: "[size=14]", close: "[/size]", placeholder: "texte" },
        ],
      },
      {
        name: t("bbcodePalette.layout"),
        tags: [
          { label: t("bbcodePalette.center"), open: "[center]", close: "[/center]", placeholder: "texte" },
          { label: t("bbcodePalette.left"), open: "[left]", close: "[/left]", placeholder: "texte" },
          { label: t("bbcodePalette.line"), open: "[hr]" },
          { label: "H1", open: "[h1]", close: "[/h1]", placeholder: "titre" },
          { label: "H2", open: "[h2]", close: "[/h2]", placeholder: "titre" },
          { label: "H3", open: "[h3]", close: "[/h3]", placeholder: "titre" },
        ],
      },
      {
        name: t("bbcodePalette.media"),
        tags: [
          { label: t("bbcodePalette.image"), open: "[img]", close: "[/img]", placeholder: "url" },
          { label: t("bbcodePalette.imgWidth"), open: "[img=300]", close: "[/img]", placeholder: "url" },
          { label: t("bbcodePalette.imgDim"), open: "[img=300x200]", close: "[/img]", placeholder: "url" },
        ],
      },
      {
        name: t("bbcodePalette.structure"),
        tags: [
          {
            label: t("bbcodePalette.table"),
            open: `[table]\n[tr]\n[th]${t("bbcodePalette.header")}[/th]\n[/tr]\n[tr]\n[td]${t("bbcodePalette.cell")}[/td]\n[/tr]\n[/table]`,
          },
          { label: "tr", open: "[tr]", close: "[/tr]" },
          { label: "td", open: "[td]", close: "[/td]", placeholder: "contenu" },
          { label: "th", open: "[th]", close: "[/th]", placeholder: "en-tête" },
        ],
      },
      {
        name: t("bbcodePalette.advanced"),
        tags: [
          { label: t("bbcodePalette.link"), open: "[url=https://]", close: "[/url]", placeholder: "texte du lien" },
          { label: t("bbcodePalette.quote"), open: "[quote]", close: "[/quote]", placeholder: "citation" },
          { label: t("bbcodePalette.spoiler"), open: "[spoiler]", close: "[/spoiler]", placeholder: "contenu caché" },
          { label: t("bbcodePalette.alert"), open: "[alert]", close: "[/alert]", placeholder: "message" },
        ],
      },
    ],
    [t],
  );

  // Use stable keys for openCats tracking
  const catKeys = ["text", "layout", "media", "structure", "advanced"];

  const toggleCat = (key: string) => {
    setOpenCats((prev) => {
      const next = new Set(prev);
      if (next.has(key)) next.delete(key);
      else next.add(key);
      return next;
    });
  };

  if (collapsed) {
    return (
      <div className="flex flex-col items-center py-2 bg-[#16213e] border-r border-[#2a2a4a] w-8 shrink-0">
        <button
          onClick={onToggle}
          className="text-gray-400 hover:text-white text-xs px-1 py-1"
          title={t("bbcodePalette.openPalette")}
        >
          ▶
        </button>
      </div>
    );
  }

  return (
    <div className="flex flex-col bg-[#16213e] border-r border-[#2a2a4a] w-48 shrink-0 overflow-y-auto">
      <div className="flex items-center justify-between px-2 py-2 border-b border-[#2a2a4a]">
        <span className="text-xs font-medium text-gray-300">{t("bbcodePalette.title")}</span>
        <button
          onClick={onToggle}
          className="text-gray-400 hover:text-white text-xs px-1"
          title={t("bbcodePalette.closePalette")}
        >
          ◀
        </button>
      </div>
      <div className="flex-1 overflow-y-auto">
        {categories.map((cat, idx) => {
          const key = catKeys[idx];
          return (
            <div key={key}>
              <button
                onClick={() => toggleCat(key)}
                className="w-full text-left px-2 py-1.5 text-xs font-semibold text-gray-400 hover:text-white hover:bg-[#1a2744] flex items-center gap-1"
              >
                <span className="text-[10px]">{openCats.has(key) ? "▾" : "▸"}</span>
                {cat.name}
              </button>
              {openCats.has(key) && (
                <div className="flex flex-wrap gap-1 px-2 pb-2">
                  {cat.tags.map((tag) => (
                    <button
                      key={tag.label}
                      onClick={() => onInsertTag(tag.open, tag.close, tag.placeholder)}
                      className="text-[11px] bg-[#2a2a4a] hover:bg-[#3a3a5a] text-gray-300 hover:text-white px-1.5 py-0.5 rounded transition-colors"
                      title={tag.close ? `${tag.open}...${tag.close}` : tag.open}
                    >
                      {tag.label}
                    </button>
                  ))}
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}
