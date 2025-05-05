
Request：
NetworkCallback -> Client -> Client.Read -> Client.Parse -> Command.Proc

Response：
NetworkCallback -> Client -> Write

Close：
Remove Client List -> Clean Event

Open：
Accept -> NewClient -> Add Event

Framework:
Wait Event -> Process Event -> Common Event

Multi Thread:
Thread#1 -> Wait Event -> Lock Range -> Process Event -> Unlock -> Check Epoch

- Server Layer：Client
- Core Layer：Command、Client、Network、Config