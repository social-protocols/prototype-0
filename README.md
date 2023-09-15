# Roadmap

## UI prototype

Top-level content container is a `Post`. `Post`s are ranked with localized upvoteRate (TODO: find better name for ranking scheme).

`Post`s have one button *reply*, with which you can add:
    - `Question` (structured)
    - `Comment`(unstructured)

`Question`s and `Comment`s can become annotations to a `Post`.
Replies are ranked with bridge-based voting.

`Question`s can have answers, which you can only rate in-context (meaning, you click on it and then you can rate it).
The `Question`s ranking only counts the ranking of the `Question`, not the answer.
`Answer`s are ranked with bridge-based ranking again.

Everything is a `Post` (`Comment`s, `Answer`s, `Post`s). `Question`s are `Question`s.
`Reply` is a `Post` with a parent.
`Comment` is a `Post` that's not a `Question`.
`Answer` is a `Reply` to a `Question`.
`Question`s can also be an `Answer`.


## Ideas

- Bridged-based ranking for questions let's the discussion group draw attention towards which information is missing and needs to be reasoned about. If everyone from different perspectives
- Comment threads can always be unstructured/non-formal, but they can be used to extract good arguments (a piece of information which changed many minds) or good questions (missing information)


## Insights

- Bayesian reasoning can work without changing people's minds. By asking them for conditionals, e.g. "Assuming X is true, do you believe Y?"
- If we want bayesian reasoning without asking for conditionals, people have to change their mind to make progress.
- if misinformation becomes popular, it's because many people actually believe it. Suppressing it does not make people learn. Instead it should become popular together with corrected information (like community notes), so everyones learns from that.

