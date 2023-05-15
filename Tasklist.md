# Tasklist

- [ ] Create a 1 to 1 translation of the C# reference implementation.
  - Layout:
    - [x] `Analyzer`
      - [x] `AnalyzerManager`
      - [x] `AnalyzerResult`
      - [x] `AnalyzerThread`
    - [ ] `Db`
      - [ ] `IDbManager`
      - [ ] `SqliteManager`
    - [ ] `FileReader`
      - [ ] `FileManager`
    - [ ] `Logging`
      - [ ] `LogLevel`
      - [ ] `Logger`
    - [ ] `Ui`
      - [x] `Ui`
      - [ ] `IOManager`
      - [ ] `CreateNewFile`
      - [ ] `UiColors`
      - [ ] `ProgressBar`
    - [-] `MainManager`

- [ ] Analyze the architecture and make changes so the application can have multiple front-ends
  - [ ] Create a front-end for the terminal
    - [ ] Select a cross-platform terminal library
  - [ ] Create a front-end for the web
    - [ ] Select a web front-end
  - [ ] Create a front-end for the desktop
    - [ ] Create a Egui front-end
    - [ ] Create a bevy front-end

