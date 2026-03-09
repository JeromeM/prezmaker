interface Props {
  onClose: () => void;
}

export default function AboutModal({ onClose }: Props) {
  return (
    <div
      className="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
      onClick={(e) => e.target === e.currentTarget && onClose()}
    >
      <div className="bg-[#1a1a2e] border border-[#2a2a4a] rounded-lg w-full max-w-md mx-4 shadow-2xl">
        <div className="flex items-center justify-between px-6 py-4 border-b border-[#2a2a4a]">
          <h2 className="text-white text-lg font-medium">A propos</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors text-xl leading-none"
          >
            &times;
          </button>
        </div>

        <div className="px-6 py-6 space-y-5">
          <div className="text-center">
            <h3 className="text-2xl font-bold text-white mb-1">PrezMaker</h3>
            <p className="text-gray-400 text-sm">v1.1.0</p>
            <p className="text-gray-500 text-xs mt-2">
              Generateur de presentations BBCode pour films, series, jeux et applications.
            </p>
          </div>

          <div className="text-center text-sm text-gray-400">
            <p>
              Developpe par{" "}
              <span className="text-white font-medium">Grommey</span>
            </p>
          </div>

          <div className="flex flex-col gap-2">
            <a
              href="https://paypal.me/grommey"
              target="_blank"
              rel="noreferrer"
              className="flex items-center justify-center gap-2 bg-[#0070ba] hover:bg-[#005ea6] text-white rounded px-4 py-2.5 text-sm font-medium transition-colors"
            >
              <svg viewBox="0 0 24 24" fill="currentColor" className="w-5 h-5">
                <path d="M7.076 21.337H2.47a.641.641 0 0 1-.633-.74L4.944.901C5.026.382 5.474 0 5.998 0h7.46c2.57 0 4.578.543 5.69 1.81 1.01 1.15 1.304 2.42 1.012 4.287-.023.143-.047.288-.077.437-.983 5.05-4.349 6.797-8.647 6.797H9.603c-.564 0-1.04.408-1.13.964L7.076 21.337z" />
              </svg>
              Donation PayPal
            </a>

            <a
              href="https://www.buymeacoffee.com/grommey"
              target="_blank"
              rel="noreferrer"
              className="flex items-center justify-center gap-2 bg-[#ffdd00] hover:bg-[#e6c800] text-[#000000] rounded px-4 py-2.5 text-sm font-medium transition-colors"
            >
              <svg viewBox="0 0 24 24" fill="currentColor" className="w-5 h-5">
                <path d="M20.216 6.415l-.132-.666c-.119-.598-.388-1.163-1.001-1.379-.197-.069-.42-.098-.57-.241-.152-.143-.196-.366-.231-.572-.065-.378-.125-.756-.192-1.133-.057-.325-.102-.69-.25-.987-.195-.4-.597-.634-.996-.788a5.723 5.723 0 0 0-.626-.194c-1-.263-2.05-.36-3.077-.416a25.834 25.834 0 0 0-3.7.062c-.915.083-1.88.184-2.75.5-.318.116-.646.256-.888.501-.297.302-.393.77-.177 1.146.154.267.415.456.692.58.36.162.737.284 1.123.366 1.075.238 2.189.331 3.287.37 1.218.05 2.437.01 3.65-.118.299-.033.598-.073.896-.119.352-.054.578-.513.474-.834-.124-.383-.457-.531-.834-.473-.466.074-.96.108-1.382.146-1.177.08-2.358.082-3.536.006a22.228 22.228 0 0 1-1.157-.107c-.086-.01-.18-.025-.258-.036-.243-.036-.484-.08-.724-.13-.111-.027-.111-.185 0-.212h.005c.277-.06.557-.108.838-.147h.002c.131-.009.263-.032.394-.048a25.076 25.076 0 0 1 3.426-.12c.674.019 1.347.067 2.017.144l.228.031c.267.04.533.088.798.145.392.085.895.113 1.07.542.055.137.08.288.111.431l.319 1.484a.237.237 0 0 1-.199.284h-.003c-.037.006-.075.01-.112.015a36.672 36.672 0 0 1-4.743.295 37.572 37.572 0 0 1-4.799-.3c-.198-.023-.551-.053-.543.266v.112c0 .108.003.217.013.325.04.594.186 1.197.539 1.69.373.52.946.855 1.572.998a8.376 8.376 0 0 0 1.429.158c.87.041 1.743.022 2.614-.01.87-.033 1.74-.084 2.607-.162.24-.022.48-.049.72-.08.372-.05.744-.105 1.114-.172.24-.044.479-.098.714-.163.382-.105.75-.27 1.007-.583.246-.298.348-.685.372-1.072.014-.244.005-.489-.024-.732l-.007-.063a.327.327 0 0 0-.023-.103l-.002-.005z" />
                <path d="M11.5 17.5c-3.038 0-5.5.672-5.5 1.5 0 .828 2.462 1.5 5.5 1.5s5.5-.672 5.5-1.5c0-.828-2.462-1.5-5.5-1.5z" opacity=".4" />
              </svg>
              Buy me a Coffee
            </a>
          </div>
        </div>

        <div className="flex justify-end px-6 py-4 border-t border-[#2a2a4a]">
          <button
            onClick={onClose}
            className="bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded text-sm transition-colors"
          >
            Fermer
          </button>
        </div>
      </div>
    </div>
  );
}
