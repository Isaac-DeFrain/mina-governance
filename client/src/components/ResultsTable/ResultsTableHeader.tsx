import React from "react";
import { useMediaQuery } from "@react-hook/media-query";

export const ResultsTableHeader = () => {
  const isMobile = useMediaQuery("only screen and (max-width: 768px)");

  if (isMobile) {
    return null;
  }

  return (
    <div className="results-grid-table-el w-full mt-[0.1rem] pb-1 border-b border-gray-7">
      <span className="grid-table-heading place-self-center">Blockheight</span>
      <span className="hidden lg:block grid-table-heading place-self-center">
        Timestamp
      </span>
      <span className="grid-table-heading place-self-center lg:place-self-left">
        Account
      </span>
      <span className="grid-table-heading place-self-center">
        Weighted Stake
      </span>
      <span className="grid-table-heading place-self-center">
        Weighted Stake %
      </span>
      <span className="grid-table-heading place-self-center">Memo</span>
      <span className="grid-table-heading place-self-center">Status</span>
    </div>
  );
};
