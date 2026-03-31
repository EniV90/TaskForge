import { useState } from "react";
import { createRoot } from "react-dom/client";
import React from "react";
import { TodoItems } from "./interfaces/toDoItems";
import getAll from "./api/get";

const App = () => {
  const [data, setData] = useState(null);
  const [error, setError] = useState(null);

  React.useEffect(() => {
    const fetchData = async () => {
      const response = await getAll();

      if (response.error) {
        setError(response.error);
      } else {
        setData(response.data);
      }
    };
    fetchData();
  }, []);

  return (
    <div>
      {error ? (
        <div style={{ color: "red" }}>Error: {error}</div>
      ) : data ? (
        <div>Data loaded: {JSON.stringify(data)}</div>
      ) : (
        <div>Loading...</div>
      )}
    </div>
  );
};

const root = createRoot(document.getElementById("root"));
root.render(<App />);
