# Luc

Luc is a project for tools that build an unstructured P2P network partially organized, that should help a little team to work together without centralised webservers.

This is a hack done especially to study P2P networks and should not be used in production.

The purpose of the idea is:
1. Network
  - All nodes are connected through an unstructured network.
  - You should know at least one peer to be connected.
2. Data sharing
  - Each peer can create, join or quit a group of peer, and the invite peers in the group (with a reasonable size that is explicitly set at the creation of a group).
  - Each group can set a level of accessibility through the network.
  - Each peer in the group are sharing a folder architecture corresponding to a git bare repository.
  - Each peer in the group provide to others peers a set of scripts that can be executed remotly.
3. Data finding
  - Each peers are providing an index of term frequency, that help the research of a piece of text.
  - You can propagate a research command through all the network or stop it when you get a response.
4. Synchronisation
  - In a group, a peer can require to others to dont do anything in this group in order to execute synchronised operations, then release the lock.


The project is in development and it remains importants things to do:
- Lock/unlock a group.
- Authorise the execution of a script.
- Creation of an orchestra to check synchronisation.
- Add the git bare sharing, need to finish first [my diff tool](https://github.com/adrien-zinger/Diff).
- Encryption in the network + create an header in TCP communications.
- Data encryption for local information, optional on starting Luc.
