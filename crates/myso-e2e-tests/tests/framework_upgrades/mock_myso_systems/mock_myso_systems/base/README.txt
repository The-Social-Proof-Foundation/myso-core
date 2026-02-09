This directory contains a mock version of the 0x3 package (myso-system).
The idea is to introduce a minimum version of the myso-system that we can use to start MySo.
We can then use this mock version as the base package to test various things such as myso system state upgrades.
This allows us to decouple from the complicated code in the original myso-system under myso-framework.
We only need to update code here and in other mock versions when the core protocol changes. This includes:
1. The genesis creation function interface
2. advance_epoch and advance_epoch_safe_mode interface
3. Any new system function call to the myso-system package required by protocol.
4. Any new information needed at epoch start.
