import { useState } from "react";
import { UserNav } from "../components/user-nav";

interface Proposal {
  id: string;
  brand: string;
  title: string;
  description: string;
  yes: number;
  no: number;
  deadline: string;
  myVote: "yes" | "no" | null;
}

const initialProposals: Proposal[] = [
  {
    id: "1",
    brand: "Le Mouton",
    title: "\uB9AC\uC6CC\uB4DC \uBC30\uC218 2\uBC30 \uC99D\uAC00",
    description:
      "\uAC77\uAE30 \uCC4C\uB9B0\uC9C0 \uB9AC\uC6CC\uB4DC\uB97C \uAE30\uC874 \uB300\uBE44 2\uBC30\uB85C \uC0C1\uD5A5\uD558\uB294 \uC81C\uC548\uC785\uB2C8\uB2E4.",
    yes: 1250,
    no: 340,
    deadline: "2026-04-15",
    myVote: null,
  },
  {
    id: "2",
    brand: "Cafe Blossom",
    title: "\uC2E0\uBA54\uB274 \uCD9C\uC2DC \uAE30\uB150 \uBCF4\uB108\uC2A4",
    description:
      "\uC2E0\uBA54\uB274 \uAD6C\uB9E4 \uC2DC \uCD94\uAC00 50 \uD3EC\uC778\uD2B8 \uC9C0\uAE09 \uC81C\uC548\uC785\uB2C8\uB2E4.",
    yes: 890,
    no: 120,
    deadline: "2026-04-20",
    myVote: null,
  },
  {
    id: "3",
    brand: "RunPulse",
    title: "\uB9C8\uB77C\uD1A4 \uC774\uBCA4\uD2B8 \uD1A0\uD070 \uBC30\uBD84",
    description:
      "\uC11C\uC6B8 \uB9C8\uB77C\uD1A4 \uCC38\uAC00\uC790\uC5D0\uAC8C 500 RPT \uBCF4\uB108\uC2A4 \uC9C0\uAE09 \uC81C\uC548\uC785\uB2C8\uB2E4.",
    yes: 2100,
    no: 180,
    deadline: "2026-04-10",
    myVote: "yes",
  },
];

export function DAOPage() {
  const [proposals, setProposals] = useState<Proposal[]>(initialProposals);
  const [toast, setToast] = useState<string | null>(null);

  const handleVote = (id: string, vote: "yes" | "no") => {
    setProposals((prev) =>
      prev.map((p) => {
        if (p.id !== id || p.myVote !== null) return p;
        return {
          ...p,
          myVote: vote,
          yes: vote === "yes" ? p.yes + 1 : p.yes,
          no: vote === "no" ? p.no + 1 : p.no,
        };
      })
    );
    setToast(vote === "yes" ? "\uCC2C\uC131 \uD22C\uD45C\uAC00 \uC644\uB8CC\uB418\uC5C8\uC2B5\uB2C8\uB2E4!" : "\uBC18\uB300 \uD22C\uD45C\uAC00 \uC644\uB8CC\uB418\uC5C8\uC2B5\uB2C8\uB2E4!");
    setTimeout(() => setToast(null), 2500);
  };

  return (
    <div className="min-h-screen bg-[#0a0e17]">
      <UserNav />

      {/* Toast */}
      {toast && (
        <div className="fixed top-20 right-4 z-50 bg-[#0f1420] border border-[#00d4aa] text-[#00d4aa] px-5 py-3 rounded-lg text-sm font-medium shadow-lg animate-[fadeIn_0.3s_ease-out]">
          {toast}
        </div>
      )}

      <div className="max-w-5xl mx-auto px-4 py-8">
        <h2 className="text-2xl font-bold text-white mb-6">
          Active Proposals
        </h2>

        <div className="space-y-4">
          {proposals.map((p) => {
            const total = p.yes + p.no;
            const yesPct = total > 0 ? (p.yes / total) * 100 : 0;
            const noPct = total > 0 ? (p.no / total) * 100 : 0;

            return (
              <div
                key={p.id}
                className="bg-[#0f1420] rounded-2xl border border-gray-800 p-6"
              >
                <div className="flex items-start justify-between mb-3">
                  <div>
                    <div className="flex items-center gap-2 mb-1">
                      <span className="px-2 py-0.5 text-xs font-medium rounded-full bg-blue-600/20 text-blue-400">
                        {p.brand}
                      </span>
                      <span className="text-xs text-gray-500">
                        Deadline: {p.deadline}
                      </span>
                    </div>
                    <h3 className="text-lg font-semibold text-white">
                      {p.title}
                    </h3>
                    <p className="text-sm text-gray-400 mt-1">
                      {p.description}
                    </p>
                  </div>
                </div>

                {/* Vote Progress */}
                <div className="mt-4 space-y-2">
                  <div className="flex items-center gap-3">
                    <span className="text-sm text-[#00d4aa] w-8">Yes</span>
                    <div className="flex-1 h-3 bg-gray-800 rounded-full overflow-hidden">
                      <div
                        className="h-full bg-[#00d4aa] rounded-full transition-all duration-500"
                        style={{ width: `${yesPct}%` }}
                      />
                    </div>
                    <span className="text-sm text-gray-400 w-20 text-right">
                      {p.yes} ({yesPct.toFixed(1)}%)
                    </span>
                  </div>
                  <div className="flex items-center gap-3">
                    <span className="text-sm text-red-400 w-8">No</span>
                    <div className="flex-1 h-3 bg-gray-800 rounded-full overflow-hidden">
                      <div
                        className="h-full bg-red-500 rounded-full transition-all duration-500"
                        style={{ width: `${noPct}%` }}
                      />
                    </div>
                    <span className="text-sm text-gray-400 w-20 text-right">
                      {p.no} ({noPct.toFixed(1)}%)
                    </span>
                  </div>
                </div>

                {/* Vote Buttons */}
                <div className="mt-4 flex items-center gap-3">
                  {p.myVote !== null ? (
                    <span className="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg bg-gray-800 text-sm text-gray-300">
                      <span className="text-[#00d4aa]">{"\u2713"}</span>
                      Voted{" "}
                      <span className="font-medium capitalize">{p.myVote}</span>
                    </span>
                  ) : (
                    <>
                      <button
                        onClick={() => handleVote(p.id, "yes")}
                        className="px-5 py-2 rounded-lg bg-[#00d4aa]/10 text-[#00d4aa] text-sm font-medium hover:bg-[#00d4aa]/20 transition-colors"
                      >
                        Vote Yes
                      </button>
                      <button
                        onClick={() => handleVote(p.id, "no")}
                        className="px-5 py-2 rounded-lg bg-red-500/10 text-red-400 text-sm font-medium hover:bg-red-500/20 transition-colors"
                      >
                        Vote No
                      </button>
                    </>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
