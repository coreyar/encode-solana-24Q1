# Monopoly

**How could decentralized version of monopoly work? What are the essential pieces of functionality?**

A decentralized game of monopoly would require minting a variety of tokens that would be controlled by a Game contract. There would be NFT player tokens, NFT tokens to represent properties, fungible tokens to represent houses, hotels, and money. Community chest and chance cards could be NFTs or managed in state depending on the trade offs.

The Game contract would keep track of what assets each player owns as well as calling an oracle to create randomness in the game for things like moving around the board, handling money and action cards. 

A turn function would allow the player to roll the die and move around the board with the Game contract automatically executing required actions. If a decision is required by a user they would call the appropriate function with their choice and the function would check they are authorized and then execute the action.

**How could people cheat?**
You need a trustworthy source of randomness because randomness doesn't exist on chain and using block properties is subject to gaming.

**How could you prevent cheating?**
Use a reliable oracle for randomness.

Authorization and validation play an important role to prevent players from taking actions out of turn.

# CBDCs

**Do you feel like CBDCs are a move towards decentralization? Will they help or hinder the adoption of other cryptocurrencies?**

Yes, I think that CBDCs are a sign of a healthy crypto ecosystem where users have multiple public and private options to choose from. It is also desirable to have a diverse mix of heavy users to represent different interests and collaborate to create the best user experience.
