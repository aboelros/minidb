import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [query, setQuery] = useState("CREATE TABLE users (\n    id INTEGER PRIMARY KEY,\n    name TEXT,\n    age INTEGER\n);\n\nINSERT INTO users VALUES (1, 'Alice', 20);\nSELECT * FROM users;");
  const [results, setResults] = useState("Ready for queries...");
  const [isExecuting, setIsExecuting] = useState(false);

  const executeQuery = async () => {
    if (!query.trim()) return;
    
    setIsExecuting(true);
    try {
      if (window.__TAURI_INTERNALS__) {
        const res: string = await invoke("execute_sql", { sql: query });
        setResults(res);
      } else {
        // Mock fallback for browser execution if Tauri backend is unavailable
        setTimeout(() => {
          setResults(`[Browser Mock Mode]\nSQL: ${query.split('\n')[0]}...\n  => Statement OK\n--------------------------------------------------\nScript execution complete.`);
          setIsExecuting(false);
        }, 500);
        return;
      }
    } catch (error) {
      setResults(`Error: ${error}`);
    } finally {
      setIsExecuting(false);
    }
  };

  return (
    <>
      <div className="panel sidebar">
        <h2>MiniDB Explorer</h2>
        <div style={{ color: "var(--text-secondary)", fontSize: "0.9rem", lineHeight: "1.6" }}>
          <p>Database Engine built from scratch in Rust.</p>
          <br />
          <p><strong>Supported Commands:</strong></p>
          <ul style={{ paddingLeft: "1.2rem", marginTop: "0.5rem" }}>
            <li>CREATE TABLE</li>
            <li>CREATE INDEX</li>
            <li>INSERT</li>
            <li>SELECT</li>
            <li>UPDATE</li>
            <li>DELETE</li>
            <li>EXPLAIN</li>
            <li>BEGIN / COMMIT / ROLLBACK</li>
          </ul>
        </div>
      </div>

      <div className="main-content">
        <div className="panel query-editor">
          <h2>Query Editor</h2>
          <textarea
            className="editor-textarea"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Enter SQL query here..."
            spellCheck="false"
          />
          <div className="actions">
            <button onClick={executeQuery} disabled={isExecuting}>
              {isExecuting ? "Executing..." : "Execute SQL"}
            </button>
          </div>
        </div>

        <div className="panel results-view">
          <h2>Execution Results</h2>
          <div className="results-output">
            {results}
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
