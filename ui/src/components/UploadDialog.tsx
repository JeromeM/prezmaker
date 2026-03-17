import { useState, useEffect, useCallback } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import type {
  PresentationMeta,
  TorrentInfo,
  C411Category,
  C411OptionType,
  C411AutoMapResult,
  C411UploadResult,
} from "../types/api";

interface Props {
  torrentPath: string;
  nfoContent: string | null;
  bbcode: string;
  meta: PresentationMeta;
  torrentInfo: TorrentInfo | null;
  onClose: () => void;
  isHtml?: boolean;
}

export default function UploadDialog({
  torrentPath,
  nfoContent,
  bbcode,
  meta,
  torrentInfo,
  onClose,
  isHtml,
}: Props) {
  const { t } = useTranslation();

  const [categories, setCategories] = useState<C411Category[]>([]);
  const [loadingCategories, setLoadingCategories] = useState(true);
  const [options, setOptions] = useState<C411OptionType[]>([]);
  const [loadingOptions, setLoadingOptions] = useState(false);

  const [title, setTitle] = useState(torrentInfo?.meta.name ?? meta.title);
  const [categoryId, setCategoryId] = useState<number>(1);
  const [subcategoryId, setSubcategoryId] = useState<number>(6);
  const [selectedOptions, setSelectedOptions] = useState<Record<number, number | number[]>>({});
  const [uploaderNote, setUploaderNote] = useState("");
  const [autoMapped, setAutoMapped] = useState<Set<number>>(new Set());

  const [uploading, setUploading] = useState(false);
  const [result, setResult] = useState<C411UploadResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  // Fetch categories on mount
  useEffect(() => {
    (async () => {
      try {
        const cats = await invoke<C411Category[]>("c411_fetch_categories");
        setCategories(cats);
      } catch (e) {
        setError(String(e));
      } finally {
        setLoadingCategories(false);
      }
    })();
  }, []);

  // Auto-map when categories load
  useEffect(() => {
    if (categories.length === 0 || !torrentInfo) return;

    (async () => {
      try {
        // First fetch options for the auto-mapped subcategory
        const mapped = await invoke<C411AutoMapResult>("c411_auto_map", {
          contentType: meta.contentType,
          parsed: torrentInfo.parsed,
          availableOptions: [],
        });

        setCategoryId(mapped.categoryId);
        setSubcategoryId(mapped.subcategoryId);

        // Fetch options for the subcategory
        await fetchAndApplyOptions(mapped.subcategoryId, torrentInfo);
      } catch (e) {
        console.error("Auto-map failed:", e);
      }
    })();
  }, [categories, torrentInfo]);

  const fetchAndApplyOptions = useCallback(
    async (subId: number, info: TorrentInfo | null) => {
      setLoadingOptions(true);
      try {
        const opts = await invoke<C411OptionType[]>("c411_fetch_options", {
          subcategoryId: subId,
        });
        setOptions(opts);

        // Auto-map options if we have parsed info
        if (info) {
          const mapped = await invoke<C411AutoMapResult>("c411_auto_map", {
            contentType: meta.contentType,
            parsed: info.parsed,
            availableOptions: opts,
          });

          const newSelected: Record<number, number | number[]> = {};
          const autoSet = new Set<number>();
          for (const [key, val] of Object.entries(mapped.options)) {
            const numKey = Number(key);
            newSelected[numKey] = val;
            autoSet.add(numKey);
          }
          setSelectedOptions(newSelected);
          setAutoMapped(autoSet);
        }
      } catch (e) {
        console.error("Fetch options failed:", e);
      } finally {
        setLoadingOptions(false);
      }
    },
    [meta.contentType],
  );

  const handleSubcategoryChange = useCallback(
    (newSubId: number) => {
      setSubcategoryId(newSubId);
      setSelectedOptions({});
      setAutoMapped(new Set());
      fetchAndApplyOptions(newSubId, torrentInfo);
    },
    [torrentInfo, fetchAndApplyOptions],
  );

  const handleOptionChange = useCallback(
    (optionTypeId: number, value: number, allowsMultiple: boolean) => {
      setSelectedOptions((prev) => {
        if (allowsMultiple) {
          const current = (prev[optionTypeId] as number[] | undefined) ?? [];
          const next = current.includes(value)
            ? current.filter((v) => v !== value)
            : [...current, value];
          return { ...prev, [optionTypeId]: next };
        }
        return { ...prev, [optionTypeId]: value };
      });
      setAutoMapped((prev) => {
        const next = new Set(prev);
        next.delete(optionTypeId);
        return next;
      });
    },
    [],
  );

  const handleUpload = useCallback(async () => {
    setUploading(true);
    setError(null);
    try {
      const optionsJson = JSON.stringify(selectedOptions);
      const res = await invoke<C411UploadResult>("c411_upload", {
        torrentPath,
        nfoContent: nfoContent ?? "",
        title,
        description: bbcode,
        categoryId,
        subcategoryId,
        optionsJson,
        uploaderNote: uploaderNote || null,
      });
      setResult(res);
    } catch (e) {
      setError(String(e));
    } finally {
      setUploading(false);
    }
  }, [torrentPath, nfoContent, title, bbcode, categoryId, subcategoryId, selectedOptions, uploaderNote]);

  // Find current category/subcategory objects
  const currentCategory = categories.find((c) =>
    c.subcategories.some((s) => s.id === subcategoryId),
  );

  const sortedOptions = [...options].sort((a, b) => a.sortOrder - b.sortOrder);

  return (
    <div
      className="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
      onMouseDown={(e) => e.target === e.currentTarget && onClose()}
    >
      <div className="bg-surface border border-edge rounded-lg w-full max-w-xl mx-4 shadow-2xl flex flex-col max-h-[85vh]">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-edge shrink-0">
          <h2 className="text-fg-bright text-lg font-medium">{t("upload.title")}</h2>
          <button
            onClick={onClose}
            className="text-fg-muted hover:text-fg-bright transition-colors text-xl leading-none"
          >
            &times;
          </button>
        </div>

        {/* Body */}
        <div className="flex-1 overflow-y-auto px-6 py-4 space-y-4">
          {result ? (
            <div className={`rounded-lg p-4 text-center ${result.success ? "bg-green-900/30 border border-green-700" : "bg-red-900/30 border border-red-700"}`}>
              <p className={result.success ? "text-green-400 text-lg font-medium" : "text-red-400 text-lg font-medium"}>
                {result.success ? t("upload.success") : t("upload.error")}
              </p>
              {result.message && (
                <p className="text-fg-muted text-sm mt-2">{result.message}</p>
              )}
            </div>
          ) : loadingCategories ? (
            <div className="flex items-center justify-center py-8 text-fg-muted">
              <svg className="animate-spin h-5 w-5 mr-2" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
              </svg>
              {t("upload.loadingCategories")}
            </div>
          ) : (
            <>
              {/* Title */}
              <div className="flex flex-col gap-1">
                <label className="text-xs text-fg-muted">{t("upload.releaseTitle")}</label>
                <input
                  type="text"
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
                />
              </div>

              {/* Category */}
              <div className="flex flex-col gap-1">
                <label className="text-xs text-fg-muted">{t("upload.category")}</label>
                <select
                  value={categoryId}
                  onChange={(e) => {
                    const newCatId = Number(e.target.value);
                    setCategoryId(newCatId);
                    const cat = categories.find((c) => c.id === newCatId);
                    if (cat && cat.subcategories.length > 0) {
                      handleSubcategoryChange(cat.subcategories[0].id);
                    }
                  }}
                  className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
                >
                  {categories.map((cat) => (
                    <option key={cat.id} value={cat.id}>
                      {cat.name}
                    </option>
                  ))}
                </select>
              </div>

              {/* Subcategory */}
              {currentCategory && (
                <div className="flex flex-col gap-1">
                  <label className="text-xs text-fg-muted">{t("upload.subcategory")}</label>
                  <select
                    value={subcategoryId}
                    onChange={(e) => handleSubcategoryChange(Number(e.target.value))}
                    className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
                  >
                    {currentCategory.subcategories.map((sub) => (
                      <option key={sub.id} value={sub.id}>
                        {sub.name}
                      </option>
                    ))}
                  </select>
                </div>
              )}

              {/* Dynamic Options */}
              {sortedOptions.length > 0 && (
                <div className="space-y-3">
                  <h3 className="text-xs text-fg-muted font-medium border-b border-edge pb-1">
                    {t("upload.options")}
                  </h3>
                  {loadingOptions ? (
                    <p className="text-fg-faint text-xs">{t("common.loading")}</p>
                  ) : (
                    sortedOptions.map((opt) => (
                      <div key={opt.id} className="flex flex-col gap-1">
                        <label className="text-xs text-fg-muted">
                          {opt.name}
                          {opt.isRequired && (
                            <span className="text-red-400 ml-1">*</span>
                          )}
                          {autoMapped.has(opt.id) && (
                            <span className="text-blue-400 ml-1 text-[10px]">
                              ({t("upload.auto")})
                            </span>
                          )}
                        </label>
                        {opt.allowsMultiple ? (
                          <div className="flex flex-wrap gap-2">
                            {opt.values
                              .sort((a, b) => a.sortOrder - b.sortOrder)
                              .map((val) => {
                                const selected = (
                                  (selectedOptions[opt.id] as number[] | undefined) ?? []
                                ).includes(val.id);
                                return (
                                  <label
                                    key={val.id}
                                    className={`flex items-center gap-1.5 text-xs px-2 py-1 rounded border cursor-pointer transition-colors ${
                                      selected
                                        ? "bg-blue-600/20 border-blue-500 text-blue-300"
                                        : "bg-input border-edge text-fg-muted hover:text-fg-bright"
                                    }`}
                                  >
                                    <input
                                      type="checkbox"
                                      checked={selected}
                                      onChange={() =>
                                        handleOptionChange(opt.id, val.id, true)
                                      }
                                      className="accent-blue-500 w-3 h-3"
                                    />
                                    {val.value}
                                  </label>
                                );
                              })}
                          </div>
                        ) : (
                          <select
                            value={(selectedOptions[opt.id] as number | undefined) ?? ""}
                            onChange={(e) =>
                              handleOptionChange(opt.id, Number(e.target.value), false)
                            }
                            className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
                          >
                            <option value="">—</option>
                            {opt.values
                              .sort((a, b) => a.sortOrder - b.sortOrder)
                              .map((val) => (
                                <option key={val.id} value={val.id}>
                                  {val.value}
                                </option>
                              ))}
                          </select>
                        )}
                      </div>
                    ))
                  )}
                </div>
              )}

              {/* Uploader Note */}
              <div className="flex flex-col gap-1">
                <label className="text-xs text-fg-muted">{t("upload.uploaderNote")}</label>
                <input
                  type="text"
                  value={uploaderNote}
                  onChange={(e) => setUploaderNote(e.target.value)}
                  placeholder={t("upload.uploaderNotePlaceholder")}
                  className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
                />
              </div>

              {/* Files summary */}
              <div className="space-y-1">
                <h3 className="text-xs text-fg-muted font-medium border-b border-edge pb-1">
                  {t("upload.files")}
                </h3>
                <div className="text-xs space-y-1">
                  <div className="flex items-center gap-2 text-fg">
                    <span className="text-green-400">&#10003;</span>
                    <span className="text-fg-muted">{t("upload.torrentFile")}</span>
                    <span className="text-fg-dim truncate flex-1" title={torrentPath}>
                      {torrentPath.split(/[/\\]/).pop()}
                    </span>
                  </div>
                  <div className="flex items-center gap-2 text-fg">
                    {nfoContent ? (
                      <span className="text-green-400">&#10003;</span>
                    ) : (
                      <span className="text-yellow-400">!</span>
                    )}
                    <span className="text-fg-muted">{t("upload.nfoFile")}</span>
                    <span className="text-fg-dim">
                      {nfoContent ? t("upload.autoGenerated") : t("upload.noNfo")}
                    </span>
                  </div>
                  <div className="flex items-center gap-2 text-fg">
                    <span className="text-green-400">&#10003;</span>
                    <span className="text-fg-muted">{isHtml ? "Description HTML" : t("upload.bbcodeFile")}</span>
                    <span className="text-fg-dim">
                      {t("upload.chars", { count: bbcode.length })}
                    </span>
                  </div>
                </div>
              </div>

              {error && (
                <div className="bg-red-900/30 border border-red-700 rounded p-3">
                  <p className="text-red-400 text-sm">{error}</p>
                </div>
              )}
            </>
          )}
        </div>

        {/* Footer */}
        <div className="flex justify-end gap-3 px-6 py-4 border-t border-edge shrink-0">
          <button
            onClick={onClose}
            className="bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded text-sm transition-colors"
          >
            {result ? t("common.close") : t("common.cancel")}
          </button>
          {!result && (
            <button
              onClick={handleUpload}
              disabled={uploading || !title.trim()}
              className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white px-4 py-2 rounded text-sm font-medium transition-colors flex items-center gap-2"
            >
              {uploading && (
                <svg className="animate-spin h-4 w-4" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                </svg>
              )}
              {uploading ? t("upload.uploading") : t("upload.upload")}
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
