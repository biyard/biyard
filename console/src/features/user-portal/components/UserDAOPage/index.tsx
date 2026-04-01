import { useState } from "react";
import { toast } from "sonner";

interface Proposal {
  id: string;
  title: string;
  description: string;
  yes_votes: number;
  no_votes: number;
  deadline: string;
  initialVote: "yes" | "no" | null;
}

const proposals: Proposal[] = [
  {
    id: "p1",
    title: "리워드 배수 2배 증가",
    description: "걷기 챌린지 리워드를 기존 대비 2배로 상향하는 제안입니다.",
    yes_votes: 1250,
    no_votes: 340,
    deadline: "April 15, 2026",
    initialVote: null,
  },
  {
    id: "p2",
    title: "자선 기부 10% 배분",
    description: "월 수익의 10%를 환경 보호 단체에 기부하는 제안입니다.",
    yes_votes: 890,
    no_votes: 560,
    deadline: "April 20, 2026",
    initialVote: null,
  },
  {
    id: "p3",
    title: "파트너 브랜드 추가 승인",
    description: "새로운 파트너 브랜드 'GreenWalk'를 플랫폼에 추가하는 제안입니다.",
    yes_votes: 2100,
    no_votes: 180,
    deadline: "April 10, 2026",
    initialVote: "yes",
  },
];

export function UserDAOPage() {
  const [votes, setVotes] = useState<Record<string, "yes" | "no">>(() => {
    const initial: Record<string, "yes" | "no"> = {};
    for (const p of proposals) {
      if (p.initialVote) {
        initial[p.id] = p.initialVote;
      }
    }
    return initial;
  });

  const [tallies, setTallies] = useState<Record<string, { yes: number; no: number }>>(() => {
    const initial: Record<string, { yes: number; no: number }> = {};
    for (const p of proposals) {
      initial[p.id] = { yes: p.yes_votes, no: p.no_votes };
    }
    return initial;
  });

  const handleVote = (proposalId: string, choice: "yes" | "no") => {
    if (votes[proposalId]) {
      toast.error("You have already voted on this proposal.");
      return;
    }

    setVotes((prev) => ({ ...prev, [proposalId]: choice }));
    setTallies((prev) => ({
      ...prev,
      [proposalId]: {
        yes: prev[proposalId].yes + (choice === "yes" ? 1 : 0),
        no: prev[proposalId].no + (choice === "no" ? 1 : 0),
      },
    }));

    const proposal = proposals.find((p) => p.id === proposalId);
    toast.success(
      `Vote cast: ${choice === "yes" ? "Yes" : "No"} on "${proposal?.title}"`
    );
  };

  return (
    <div>
      {/* Header */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          DAO Governance
        </h1>
        <p className="mt-1 text-gray-600 dark:text-gray-400">
          Vote on active proposals to shape the community's future.
        </p>
      </div>

      {/* Proposal Cards */}
      <div className="space-y-6">
        {proposals.map((proposal) => {
          const t = tallies[proposal.id];
          const total = t.yes + t.no;
          const yesPercent = total > 0 ? Math.round((t.yes / total) * 100) : 0;
          const noPercent = total > 0 ? Math.round((t.no / total) * 100) : 0;
          const userVote = votes[proposal.id];

          return (
            <div
              key={proposal.id}
              className="bg-white dark:bg-gray-800 rounded-xl shadow p-6"
            >
              {/* Title & Description */}
              <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-1">
                {proposal.title}
              </h2>
              <p className="text-sm text-gray-600 dark:text-gray-400 mb-5">
                {proposal.description}
              </p>

              {/* Yes Bar */}
              <div className="mb-3">
                <div className="flex items-center justify-between mb-1">
                  <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                    Yes
                  </span>
                  <span className="text-sm text-gray-500 dark:text-gray-400">
                    {t.yes.toLocaleString()} votes
                  </span>
                </div>
                <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-5 overflow-hidden">
                  <div
                    className="bg-emerald-500 h-5 rounded-full flex items-center justify-end pr-2 transition-all duration-500"
                    style={{ width: `${Math.max(yesPercent, 2)}%` }}
                  >
                    {yesPercent >= 12 && (
                      <span className="text-xs font-semibold text-white">
                        {yesPercent}%
                      </span>
                    )}
                  </div>
                </div>
                {yesPercent < 12 && (
                  <span className="text-xs font-semibold text-gray-500 dark:text-gray-400 ml-1">
                    {yesPercent}%
                  </span>
                )}
              </div>

              {/* No Bar */}
              <div className="mb-5">
                <div className="flex items-center justify-between mb-1">
                  <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                    No
                  </span>
                  <span className="text-sm text-gray-500 dark:text-gray-400">
                    {t.no.toLocaleString()} votes
                  </span>
                </div>
                <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-5 overflow-hidden">
                  <div
                    className="bg-red-500 h-5 rounded-full flex items-center justify-end pr-2 transition-all duration-500"
                    style={{ width: `${Math.max(noPercent, 2)}%` }}
                  >
                    {noPercent >= 12 && (
                      <span className="text-xs font-semibold text-white">
                        {noPercent}%
                      </span>
                    )}
                  </div>
                </div>
                {noPercent < 12 && (
                  <span className="text-xs font-semibold text-gray-500 dark:text-gray-400 ml-1">
                    {noPercent}%
                  </span>
                )}
              </div>

              {/* Footer: Deadline + Vote Buttons */}
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-500 dark:text-gray-400">
                  Deadline:{" "}
                  <span className="font-medium text-gray-700 dark:text-gray-300">
                    {proposal.deadline}
                  </span>
                </span>

                <div className="flex gap-2">
                  {userVote ? (
                    <span className="text-sm font-medium text-gray-500 dark:text-gray-400 italic">
                      You voted:{" "}
                      <span
                        className={
                          userVote === "yes"
                            ? "text-emerald-600 dark:text-emerald-400"
                            : "text-red-600 dark:text-red-400"
                        }
                      >
                        {userVote === "yes" ? "Yes" : "No"}
                      </span>
                    </span>
                  ) : (
                    <>
                      <button
                        onClick={() => handleVote(proposal.id, "yes")}
                        className="px-4 py-2 text-sm font-medium rounded-lg border border-emerald-500 text-emerald-600 dark:text-emerald-400 hover:bg-emerald-50 dark:hover:bg-emerald-900/20 transition-colors"
                      >
                        Vote Yes
                      </button>
                      <button
                        onClick={() => handleVote(proposal.id, "no")}
                        className="px-4 py-2 text-sm font-medium rounded-lg border border-red-500 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
                      >
                        Vote No
                      </button>
                    </>
                  )}
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
