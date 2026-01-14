import { useState, useMemo, useEffect } from "react";
import { Loader2, Coins, Plus, Star } from "lucide-react";
import type { Project } from "../../models/project";
import type { Token } from "../../../tokens/models/token";
import type { PointTransaction } from "../../../points/models/point-transaction";
import type { TabProps } from "./types";
import { useGetPointAggregation } from "../../../points/hooks/use-get-point-aggregation";

interface OverviewTabProps extends TabProps {
  project: Project;
  projectId: string;
  token?: Token;
  isLoadingToken: boolean;
  pointTransactions: PointTransaction[];
  isLoadingPointTransactions: boolean;
}

export function OverviewTab({
  project,
  projectId,
  token,
  isLoadingToken,
  pointTransactions,
  isLoadingPointTransactions,
  t,
}: OverviewTabProps) {
  // Get unique months from transactions, sorted descending
  const availableMonths = useMemo(() => {
    const months = [...new Set(pointTransactions.map((tx) => tx.month))];
    return months.sort((a, b) => b.localeCompare(a));
  }, [pointTransactions]);

  const [selectedMonth, setSelectedMonth] = useState<string>("");

  // Set initial month when available months change
  useEffect(() => {
    if (availableMonths.length > 0 && !selectedMonth) {
      setSelectedMonth(availableMonths[0]);
    }
  }, [availableMonths, selectedMonth]);

  // Fetch aggregation data for selected month
  const { data: aggregation, isLoading: isLoadingAggregation } =
    useGetPointAggregation(projectId, selectedMonth);

  return (
    <div className="space-y-6">
      {/* Project Overview Card (merged with Token Configuration) */}
      <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
        <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
          {t.overview}
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* Basic Info */}
          <dl className="space-y-4">
            <div>
              <dt className="text-sm font-medium text-gray-500 dark:text-gray-400">
                {t.projectId}
              </dt>
              <dd className="mt-1">
                <code className="text-sm text-gray-900 dark:text-white bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded">
                  {project.id}
                </code>
              </dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-gray-500 dark:text-gray-400">
                {t.createdAt}
              </dt>
              <dd className="mt-1 text-sm text-gray-900 dark:text-white">
                {project.getFormattedCreatedAt()}
              </dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-gray-500 dark:text-gray-400">
                {t.updatedAt}
              </dt>
              <dd className="mt-1 text-sm text-gray-900 dark:text-white">
                {project.getFormattedUpdatedAt()}
              </dd>
            </div>
          </dl>

          {/* Token Configuration */}
          <dl className="space-y-4">
            <div>
              <dt className="text-sm font-medium text-gray-500 dark:text-gray-400">
                {t.monthlyTokenSupply}
              </dt>
              <dd className="mt-1 text-sm text-gray-900 dark:text-white">
                {project.getFormattedTokenSupply()}
              </dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-gray-500 dark:text-gray-400">
                {t.exchangeRatio}
              </dt>
              <dd className="mt-1 text-sm text-gray-900 dark:text-white">
                1 : {project.exchangeRatio}
              </dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-gray-500 dark:text-gray-400">
                {t.tokenValue}
              </dt>
              <dd className="mt-1 text-sm text-gray-900 dark:text-white">-</dd>
            </div>
          </dl>
        </div>
      </div>

      {/* Token Info Card */}
      <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
        <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
          {t.tokenInfo}
        </h3>
        {isLoadingToken ? (
          <div className="flex items-center justify-center py-8">
            <Loader2 className="h-8 w-8 text-gray-400 animate-spin" />
          </div>
        ) : !token ? (
          <div className="text-center py-8">
            <Coins className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
              {t.noTokens}
            </h3>
            <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
              {t.noTokensDescription}
            </p>
            <div className="mt-6">
              <button className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700">
                <Plus className="h-5 w-5 mr-2" />
                {t.createToken}
              </button>
            </div>
          </div>
        ) : (
          <div>
            <div className="flex items-center space-x-4 mb-6">
              <div className="p-3 bg-blue-100 dark:bg-blue-900 rounded-full">
                <Coins className="h-8 w-8 text-blue-600 dark:text-blue-400" />
              </div>
              <div>
                <h4 className="text-xl font-semibold text-gray-900 dark:text-white">
                  {token.name}
                </h4>
                <span className="px-2 py-1 text-xs font-semibold bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded">
                  {token.symbol}
                </span>
              </div>
            </div>

            {token.description && (
              <p className="text-gray-500 dark:text-gray-400 mb-6">
                {token.description}
              </p>
            )}

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
                <dt className="text-sm font-medium text-gray-500 dark:text-gray-400">
                  {t.totalSupply}
                </dt>
                <dd className="mt-1 text-2xl font-semibold text-gray-900 dark:text-white">
                  {token.getFormattedTotalSupply()}
                </dd>
              </div>
              <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
                <dt className="text-sm font-medium text-gray-500 dark:text-gray-400">
                  Circulating Supply
                </dt>
                <dd className="mt-1 text-2xl font-semibold text-gray-900 dark:text-white">
                  {token.getFormattedCirculatingSupply()}
                </dd>
              </div>
              <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
                <dt className="text-sm font-medium text-gray-500 dark:text-gray-400">
                  Decimals
                </dt>
                <dd className="mt-1 text-2xl font-semibold text-gray-900 dark:text-white">
                  {token.decimals}
                </dd>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Point Info Card */}
      <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-medium text-gray-900 dark:text-white">
            {t.pointInfo}
          </h3>
          {availableMonths.length > 0 && (
            <select
              value={selectedMonth}
              onChange={(e) => setSelectedMonth(e.target.value)}
              className="text-sm border border-gray-300 dark:border-gray-600 rounded-md px-3 py-1.5 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              {availableMonths.map((month) => (
                <option key={month} value={month}>
                  {month}
                </option>
              ))}
            </select>
          )}
        </div>
        {isLoadingPointTransactions || isLoadingAggregation ? (
          <div className="flex items-center justify-center py-8">
            <Loader2 className="h-8 w-8 text-gray-400 animate-spin" />
          </div>
        ) : !aggregation ? (
          <div className="text-center py-8">
            <Star className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
              {t.noPointsYet}
            </h3>
            <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
              {t.noPointsDescription}
            </p>
          </div>
        ) : (
          <div>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
              <div className="bg-green-50 dark:bg-green-900/20 rounded-lg p-4">
                <dt className="text-sm font-medium text-green-600 dark:text-green-400">
                  {t.totalAwarded}
                </dt>
                <dd className="mt-1 text-2xl font-semibold text-green-700 dark:text-green-300">
                  {aggregation.getFormattedAwarded()}
                </dd>
              </div>
              <div className="bg-red-50 dark:bg-red-900/20 rounded-lg p-4">
                <dt className="text-sm font-medium text-red-600 dark:text-red-400">
                  {t.totalDeducted}
                </dt>
                <dd className="mt-1 text-2xl font-semibold text-red-700 dark:text-red-300">
                  {aggregation.getFormattedDeducted()}
                </dd>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
