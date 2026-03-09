import { useState } from "react";

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

const CATEGORIES: Category[] = [
  {
    name: "Texte",
    tags: [
      { label: "Gras", open: "[b]", close: "[/b]", placeholder: "texte" },
      { label: "Italique", open: "[i]", close: "[/i]", placeholder: "texte" },
      { label: "Souligné", open: "[u]", close: "[/u]", placeholder: "texte" },
      { label: "Couleur", open: '[color=#FF0000]', close: "[/color]", placeholder: "texte" },
      { label: "Taille", open: "[size=14]", close: "[/size]", placeholder: "texte" },
    ],
  },
  {
    name: "Mise en page",
    tags: [
      { label: "Centrer", open: "[center]", close: "[/center]", placeholder: "texte" },
      { label: "Gauche", open: "[left]", close: "[/left]", placeholder: "texte" },
      { label: "Ligne", open: "[hr]" },
      { label: "H1", open: "[h1]", close: "[/h1]", placeholder: "titre" },
      { label: "H2", open: "[h2]", close: "[/h2]", placeholder: "titre" },
      { label: "H3", open: "[h3]", close: "[/h3]", placeholder: "titre" },
    ],
  },
  {
    name: "Média",
    tags: [
      { label: "Image", open: "[img]", close: "[/img]", placeholder: "url" },
      { label: "Img (larg.)", open: "[img=300]", close: "[/img]", placeholder: "url" },
      { label: "Img (dim.)", open: "[img=300x200]", close: "[/img]", placeholder: "url" },
    ],
  },
  {
    name: "Structure",
    tags: [
      {
        label: "Table",
        open: "[table]\n[tr]\n[th]En-tête[/th]\n[/tr]\n[tr]\n[td]Cellule[/td]\n[/tr]\n[/table]",
      },
      { label: "tr", open: "[tr]", close: "[/tr]" },
      { label: "td", open: "[td]", close: "[/td]", placeholder: "contenu" },
      { label: "th", open: "[th]", close: "[/th]", placeholder: "en-tête" },
    ],
  },
  {
    name: "Avancé",
    tags: [
      { label: "Lien", open: "[url=https://]", close: "[/url]", placeholder: "texte du lien" },
      { label: "Citation", open: "[quote]", close: "[/quote]", placeholder: "citation" },
      { label: "Spoiler", open: "[spoiler]", close: "[/spoiler]", placeholder: "contenu caché" },
      { label: "Alerte", open: "[alert]", close: "[/alert]", placeholder: "message" },
    ],
  },
];

interface Props {
  collapsed: boolean;
  onToggle: () => void;
  onInsertTag: (open: string, close?: string, placeholder?: string) => void;
}

export default function BBCodePalette({ collapsed, onToggle, onInsertTag }: Props) {
  const [openCats, setOpenCats] = useState<Set<string>>(new Set(["Texte"]));

  const toggleCat = (name: string) => {
    setOpenCats((prev) => {
      const next = new Set(prev);
      if (next.has(name)) next.delete(name);
      else next.add(name);
      return next;
    });
  };

  if (collapsed) {
    return (
      <div className="flex flex-col items-center py-2 bg-[#16213e] border-r border-[#2a2a4a] w-8 shrink-0">
        <button
          onClick={onToggle}
          className="text-gray-400 hover:text-white text-xs px-1 py-1"
          title="Ouvrir la palette BBCode"
        >
          ▶
        </button>
      </div>
    );
  }

  return (
    <div className="flex flex-col bg-[#16213e] border-r border-[#2a2a4a] w-48 shrink-0 overflow-y-auto">
      <div className="flex items-center justify-between px-2 py-2 border-b border-[#2a2a4a]">
        <span className="text-xs font-medium text-gray-300">Balises</span>
        <button
          onClick={onToggle}
          className="text-gray-400 hover:text-white text-xs px-1"
          title="Replier la palette"
        >
          ◀
        </button>
      </div>
      <div className="flex-1 overflow-y-auto">
        {CATEGORIES.map((cat) => (
          <div key={cat.name}>
            <button
              onClick={() => toggleCat(cat.name)}
              className="w-full text-left px-2 py-1.5 text-xs font-semibold text-gray-400 hover:text-white hover:bg-[#1a2744] flex items-center gap-1"
            >
              <span className="text-[10px]">{openCats.has(cat.name) ? "▾" : "▸"}</span>
              {cat.name}
            </button>
            {openCats.has(cat.name) && (
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
        ))}
      </div>
    </div>
  );
}
