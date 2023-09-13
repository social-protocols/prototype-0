-- Inserting the first post
INSERT INTO post (id, content) VALUES (1, "The reason I *know* capitalism is inefficient is that people work much better via intrinsic motivation than extrinsic motivation AND it is very rare / nearly impossible to align extrinsic/intrinsic incentives in a totally abstracted race to the bottom economic framework.");

-- Inserting the replies
INSERT INTO post (content, parent) VALUES 
("If we had a better education system that empowered people to offer their gifts & a resource-based economy that maps flow of exchange, extrinsic and intrinsic incentives could be aligned. It’s a design challenge. Let’s reject capitalist realism and create a more beautiful world!", 1),
("And yet intrinsic motivation isn’t always sufficient to produce cooperation at scale. I see it mostly like a layer cake: You need the base to be a stable structure for the provision of basic needs irrespective of how it makes you feel in the moment. If that works, you get to build intricate structures of artistic expression on top that produce interest in the form and look of the cake. A select few get to fixate their eyes on the cherry on top, which is complete realization of internal purpose in their daily lived reality. I do think we can produce much greater alignment of internal and external worlds, but it’s not by privileging the internal world. Instead, it’s by continuously seesawing between building in one and balancing it out by shifting focus to the other, and back again.", 1),
("Agree, but it helps to have all survival bases covered first to be able to be intrinsically motivated :)", 1),
("Capitalism (maybe you mean the free market aspect) requires people have strong intrinsic motivation to fullfill the demand in the long term. Extrinsic motivation is the superficial/external view of the intrinsic that is behind. Both are inseparable aspects of motivation", 1),
("I see some people mocking this take, but I think that may be because of the focus on the standard work paradigm. The future of work is going to be far different than the corporations of today. Very soon, you're going to begin seeing companies of <20 employees running circles around the monoliths of today. Those teams of twenty will have equity ownership in their organization, will have high intrinsic motivation, and they're going to absolutely love what they do. Compare that to many of today's working environments where many (not all) employees are 'working for the weekend,' doing the minimum to survive, sometimes putting in three hours of real productive work in an eight hour shift. Add to that the fact that monoliths move slow, are cautious, willing to take fewer risks, and have little motivation to make moonshots. When you begin to think about what an organization that leverages emerging tech could look like, compared to what we have today. You'll realize Benjamin is on to something. I don't know if we'll come to see this model as an inefficiency in capitalism or if we'll gain a more nuanced understanding of the root cause of our problems today. But economic frameworks and our relationship to work is going to change. There's no doubt. It may take 40 years to begin to fully realize that change, or see it as a mainstream adoption among our culture at large, but it is going to change. So before a knee-jerk reaction, think about the relationship you'd like to have with your work and place of employment. Then go one step further and imagine how you could begin making that change to live a more fulfilling lifestyle by the end of next year.", 1);