**5.11** **Processes**


Processes are used to describe the decoding of syntax elements. A process has a separate specification and invoking. All
syntax elements and upper case variables that pertain to the current syntax structure and depending syntax structures are
available in the process specification and invoking. A process specification may also have a lower case variable explicitly
specified as the input. Each process specification has explicitly specified an output. The output is a variable that can either
be an upper case variable or a lower case variable.


When invoking a process, the assignment of variables is specified as follows:

      - If the variables at the invoking and the process specification do not have the same name, the variables are
explicitly assigned to lower case input or output variables of the process specification.

      - Otherwise (the variables at the invoking and the process specification have the same name), assignment is
implied.


In the specification of a process, a specific macroblock may be referred to by the variable name having a value equal to
the address of the specific macroblock.


**6** **Source, coded, decoded and output data formats, scanning processes, and neighbouring**
**relationships**
