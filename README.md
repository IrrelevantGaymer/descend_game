Descend is a 2 player strategy game where players will place pieces on a board to form lines.
To win, you must have the longest line of your color.

## Rules ##

### Players and the Board ###

One player is red and the other player is black.
Each player has 3 Aces, 3 Kings, 3 Queens, and 3 Jacks.

The board is a 13x13 grid starting off with a Wild piece.
The Wild piece is both colors, so both players can use the Wild piece in their lines.

Each player will take turns placing down one of their pieces.  
In order to place a piece, it must be orthogonally connected to a piece of higher (not equal) value.
The pieces in order from highest to lowest: Wild > Ace > King > Queen > Jack

### Placing Pieces ###

For example, you can place an Ace, King, Queen, or Jack against a Wild, but you can only place a Queen or Jack against a King.
If a space is orthogonally touching more than one piece, the highest value piece takes precedence.
For example, if a space is touching a King and a Queen, you can still place a Queen there 
because the King takes precedence and the Queen is of lower value than the King.

### Blocked Spaces ###

When a piece is placed, spaces can be blocked.  For any piece orthogonally touching the placed piece, 
the first empty space in the touching piece's direction is blocked for your opponent's next move.
Multiple spaces can be blocked given multiple pieces are touching the placed piece.
There is one exception, when you place a piece next to a Wild and there is no other piece in that direction, no space is blocked.

### Free Pieces ###

If there is no legal move for a type of piece, that type of piece can now be places anywhere excluding blocked spaces.
Usually Aces will become free because the Wild is the only piece you can place Aces against.

### Winning ###

The game ends when both players run out of pieces to place.  Once the game concludes,
you will count up the length of all cardinal and diagonal lines both players have formed.  
The player with the longest line wins.  If both player's longest line has the same length,
you break the tie by comparing the next longest line, and the next longest line, and the next
until either a winner has been decided or there are no more lines resulting in a draw.

### Controls ###

You can scroll through moves via the keyboard. \
F = First Move \
P = Previous Move \
N = Next Move \
L = Last Move
