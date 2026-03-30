import { useState } from "react";
import { useMode } from "@/contexts/ModeContext";
import { getDAOProposals, type DAOProposal } from "@/lib/mock-data";
import { toast } from "sonner";

export function DAOPage() {
  const { isAdmin } = useMode();
  const proposals = getDAOProposals();
  const [votes, setVotes] = useState<Record<string, "yes" | "no">>({});
  const [localTallies, setLocalTallies] = useState<
    Record<string, { yes: number; no: number }>
  >({});

  const getTallies = (proposal: DAOProposal) => {
    const override = localTallies[proposal.id];
    return {
      yes: override ? override.yes : proposal.yes_votes,
      no: override ? override.no : proposal.no_votes,
    };
  };

  const handleVote = (proposalId: string, choice: "yes" | "no") => {
    if (votes[proposalId]) {
      toast.error("You have already voted on this proposal.");
      return;
    }

    const proposal = proposals.find((p) => p.id === proposalId);
    if (!proposal) return;

    const currentTallies = getTallies(proposal);

    setVotes((prev) => ({ ...prev, [proposalId]: choice }));
    setLocalTallies((prev) => ({
      ...prev,
      [proposalId]: {
        yes: currentTallies.yes + (choice === "yes" ? 1 : 0),
        no: currentTallies.no + (choice === "no" ? 1 : 0),
      },
    }));

    toast.success(
      `Vote cast: ${choice === "yes" ? "Yes" : "No"} on "${proposal.title}"`
    );
  };

  return (
    <div>
      {/* Page Header */}
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          DAO Governance
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          {isAdmin
            ? "Review and monitor proposal voting activity."
            : "Vote on active proposals to shape the project's future."}
        </p>
      </div>

      {/* Proposals List */}
      <div className="space-y-6">
        {proposals.map((proposal) => {
          const tallies = getTallies(proposal);
          const totalVotes = tallies.yes + tallies.no;
          const yesPercent =
            totalVotes > 0 ? Math.round((tallies.yes / totalVotes) * 100) : 0;
          const noPercent =
            totalVotes > 0 ? Math.round((tallies.no / totalVotes) * 100) : 0;
          const userVote = votes[proposal.id];

          return (
            <div
              key={proposal.id}
              className="bg-white dark:bg-gray-800 shadow rounded-lg p-6"
            >
              {/* Title & Description */}
              <div className="mb-4">
                <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
                  {proposal.title}
                </h2>
                <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
                  {proposal.description}
                </p>
              </div>

              {/* Vote Bars */}
              <div className="space-y-3 mb-4">
                {/* Yes Bar */}
                <div>
                  <div className="flex items-center justify-between mb-1">
                    <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Yes
                    </span>
                    <span className="text-sm text-gray-500 dark:text-gray-400">
                      {tallies.yes.toLocaleString()} votes
                    </span>
                  </div>
                  <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-6 overflow-hidden">
                    <div
                      className="bg-emerald-500 h-6 rounded-full flex items-center justify-end pr-2 transition-all duration-500"
                      style={{ width: `${Math.max(yesPercent, 0)}%` }}
                    >
                      {yesPercent >= 10 && (
                        <span className="text-xs font-semibold text-white">
                          {yesPercent}%
                        </span>
                      )}
                    </div>
                    {yesPercent < 10 && (
                      <span className="text-xs font-semibold text-gray-600 dark:text-gray-300 ml-2 -mt-6 block">
                        {yesPercent}%
                      </span>
                    )}
                  </div>
                </div>

                {/* No Bar */}
                <div>
                  <div className="flex items-center justify-between mb-1">
                    <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      No
                    </span>
                    <span className="text-sm text-gray-500 dark:text-gray-400">
                      {tallies.no.toLocaleString()} votes
                    </span>
                  </div>
                  <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-6 overflow-hidden">
                    <div
                      className="bg-red-500 h-6 rounded-full flex items-center justify-end pr-2 transition-all duration-500"
                      style={{ width: `${Math.max(noPercent, 0)}%` }}
                    >
                      {noPercent >= 10 && (
                        <span className="text-xs font-semibold text-white">
                          {noPercent}%
                        </span>
                      )}
                    </div>
                    {noPercent < 10 && (
                      <span className="text-xs font-semibold text-gray-600 dark:text-gray-300 ml-2 -mt-6 block">
                        {noPercent}%
                      </span>
                    )}
                  </div>
                </div>
              </div>

              {/* Deadline */}
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-500 dark:text-gray-400">
                  Deadline:{" "}
                  <span className="font-medium text-gray-700 dark:text-gray-300">
                    {new Date(proposal.deadline).toLocaleDateString("en-US", {
                      year: "numeric",
                      month: "long",
                      day: "numeric",
                    })}
                  </span>
                </span>

                {/* Vote Buttons (User Mode Only) */}
                {!isAdmin && (
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
                )}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
