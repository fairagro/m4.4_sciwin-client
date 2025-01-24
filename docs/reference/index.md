# Reference
SciWIn client provides commands for project initialization ([`s4n init`](init.md)), working with CWL CommandLineTools ([`s4n tool`](tool.md)) and CWL Workflows ([`s4n workflow`](workflow.md)), metadata annotation ([`s4n annotate`](annotate.md)), the execution of CWL ([`s4n execute`](execute.md)) and synchronization with a remote sever ([`s4n sync`](sync.md)).

!!! abstract "Usage"
    ```
    Client tool for Scientific Workflow Infrastructure (SciWIn)

    Usage: s4n <COMMAND>

    Commands:
      init      Initializes project folder structure and repository
      tool      Provides commands to create and work with CWL CommandLineTools
      workflow  Provides commands to create and work with CWL Workflows
      annotate  
      execute   Execution of CWL Files locally or on remote servers [aliases: ex]
      sync      
      help      Print this message or the help of the given subcommand(s)

    Options:
      -h, --help     Print help
      -V, --version  Print version
    ```