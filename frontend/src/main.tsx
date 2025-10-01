import { StrictMode, useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import CardList from "./CardList.tsx";
import SeriesFilter from "./SeriesFilter.tsx";
import { getSeries, type Series } from "./services/cards.ts";
import { ToastContainer } from "react-toastify";

const App = () => {
  const [series, setSeries] = useState<Series[]>([]);
  const [selectedSeries, setSelectedSeries] = useState<Series | null>(null);

  const handleChangeSelection = (series: Series) => {
    if (!selectedSeries) {
      setSelectedSeries(series);
      return;
    }
    const newSelection = series.id === selectedSeries!.id ? null : series;
    setSelectedSeries(newSelection);
  };

  const fetchSeries = async () => {
    const data = await getSeries();

    setSeries(data);
  };

  useEffect(() => {
    fetchSeries();
  }, []);

  return (
    <>
      <ToastContainer theme={"colored"} />
      <div className="container mx-auto min-h-screen">
        <h1 className="text-2xl font-bold mb-4">Card Collection</h1>

        {/* Parent grid with 2 columns */}
        <div className="grid grid-cols-12 gap-4">
          {/* Left column */}
          <div className="col-span-3">
            <SeriesFilter
              currentSelection={selectedSeries}
              series={series}
              onSelect={(id) => {
                const clickedSeries = series.filter((s) => s.id === id)[0];
                handleChangeSelection(clickedSeries);
              }}
            />
          </div>

          {/* Right column */}
          <div className="col-span-9">
            <CardList series={selectedSeries} />
          </div>
        </div>
      </div>
    </>
  );
};

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>
);
