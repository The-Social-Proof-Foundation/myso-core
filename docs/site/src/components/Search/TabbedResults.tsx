// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

import React from "react";

export default function TabbedResults({
  activeTab,
  onChange,
  tabs,
  showTooltips = true,
}) {
  const suitooltip = "Search results from the official MySo Docs";
  const suinstooltip = "Search results from MySo Name Service";
  const movetooltip = "Search results from The Move Book";
  const dapptooltip = "Search results from the MySo ecosystem SDKs";
  const walrustooltip =
    "Search results from the Walrus decentralized storage platform";
  return (
    <div className="mb-4 flex justify-start border-2 border-solid border-white rounded-t-lg dark:bg-black dark:border-myso-black border-b-myso-gray-50 dark:border-b-myso-gray-80">
      {tabs.map(({ label, indexName, count }) => (
        <div className="relative group inline-block" key={indexName}>
          <button
            className={`mr-4 flex items-center font-semibold text-sm lg:text-md xl:text-lg bg-white dark:bg-myso-black cursor-pointer dark:text-myso-gray-45 ${activeTab === indexName ? "text-myso-disabled/100 font-bold border-2 border-solid border-transparent border-b-myso-blue-dark dark:border-b-myso-blue" : "border-transparent text-myso-disabled/70"}`}
            onClick={() => onChange(indexName)}
          >
            {label}{" "}
            <span
              className={`dark:text-myso-gray-90 text-xs rounded-full ml-1 py-1 px-2 border border-solid ${activeTab === indexName ? "dark:!text-myso-gray-45 bg-transparent border-myso-gray-3s dark:border-myso-gray-50" : "bg-myso-gray-45 border-transparent"}`}
            >
              {count}
            </span>
          </button>
          {showTooltips && (
            <div className="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 w-max max-w-xs px-2 py-1 text-sm text-white bg-gray-800 rounded tooltip-delay">
              {label === "MySo"
                ? suitooltip
                : label === "SuiNS"
                  ? suinstooltip
                  : label === "The Move Book"
                    ? movetooltip
                    : label === "SDKs"
                      ? dapptooltip
                      : walrustooltip}
            </div>
          )}
        </div>
      ))}
    </div>
  );
}
