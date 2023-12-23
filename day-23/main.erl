-module(main).
-export([main/0]).

long_walk(Input) ->
    ParsedGrid = parse_grid(Input),
    {Entry, End} = {{1, 2}, {length(ParsedGrid), length(lists:nth(1, ParsedGrid)) - 1}},
    Points = add_points(ParsedGrid),
    Graph = create_graph(Points, ParsedGrid),
    dfs(Graph, Entry, End).

add_points(Grid) ->
    Rows = length(Grid),
    Columns = length(hd(Grid)),
    Coordinates = [{R, C} || R <- lists:seq(1, Rows), C <- lists:seq(1, Columns)],
    Points = lists:filter(fun({R, C}) -> is_valid_point(R, C, Grid) end, Coordinates),
    Points.

is_valid_point(R, C, Grid) ->
    case get_cell(R, C, Grid) of
        '#' -> false;
        _ -> count_neighbors(R, C, Grid) >= 3
    end.

count_neighbors(R, C, Grid) ->
    Neighbors = [{R - 1, C}, {R + 1, C}, {R, C - 1}, {R, C + 1}],
    lists:foldl(
        fun({NR, NC}, Acc) ->
            case get_cell(NR, NC, Grid) of
                '#' -> Acc;
                _ -> Acc + 1
            end
        end,
        0,
        Neighbors
    ).

get_cell(R, C, Grid) ->
    case lists:nth(R, Grid) of
        Row when is_list(Row) -> lists:nth(C, Row);
        _ -> '#'
    end.

create_graph(Points, Grid) ->
    create_graph(Points, Points, Grid, #{}).
create_graph([], _AllPoints, _Grid, Graph) ->
    Graph;
create_graph([Pt | Rest], AllPoints, Grid, Graph) ->
    NewGraph = explore_point(Pt, Pt, AllPoints, Grid, #{}, 0, []),
    create_graph(Rest, AllPoints, Grid, maps:merge(Graph, NewGraph)).

explore_point(_StartPt, _CurrentPt, _AllPoints, _Grid, Graph, _N, []) ->
    Graph;
explore_point(StartPt, _CurrentPt, AllPoints, Grid, Graph, N, [NextPt | Rest]) ->
    {NR, NC} = NextPt,
    case lists:member(NextPt, AllPoints) andalso NextPt =/= StartPt of
        true ->
            UpdatedGraph = update_graph(StartPt, NextPt, N, Graph),
            explore_point(StartPt, NextPt, AllPoints, Grid, UpdatedGraph, N, Rest);
        false ->
            Directions = get_directions(Grid, NR, NC),
            NewPts = get_new_points(NR, NC, Directions, Grid),
            explore_point(StartPt, NextPt, AllPoints, Grid, Graph, N + 1, NewPts ++ Rest)
    end.

update_graph(From, To, Weight, Graph) ->
    case maps:find(From, Graph) of
        {ok, Connections} ->
            maps:update(From, maps:put(To, Weight, Connections), Graph);
        error ->
            maps:put(From, #{To => Weight}, Graph)
    end.

get_directions(Grid, R, C) ->
    case get_cell(R, C, Grid) of
        '^' -> [{-1, 0}];
        'v' -> [{1, 0}];
        '<' -> [{0, -1}];
        '>' -> [{0, 1}];
        '.' -> [{-1, 0}, {1, 0}, {0, -1}, {0, 1}];
        _ -> []
    end.
% On part2 just change to it
% get_directions(Grid, R, C) ->
%     [{-1, 0}, {1, 0}, {0, -1}, {0, 1}].

get_new_points(R, C, Directions, Grid) ->
    [{R + DR, C + DC} || {DR, DC} <- Directions, is_valid_cell(R + DR, C + DC, Grid)].

is_valid_cell(R, C, Grid) ->
    GridRows = length(Grid),
    GridColumns = length(hd(Grid)),
    R >= 0 andalso R < GridRows andalso C >= 0 andalso C < GridColumns.

parse_grid(Input) ->
    Lines = string:split(binary_to_list(Input), "\n", all),
    [Line || Line <- Lines].

dfs(Graph, Start, End) ->
    dfs(Graph, Start, End, #{}, -infinity).

dfs(_Graph, End, End, _Seen, _Max) ->
    0;
dfs(Graph, Pt, End, Seen, Max) ->
    case maps:is_key(Pt, Seen) of
        true ->
            Max;
        false ->
            NewSeen = maps:put(Pt, true, Seen),
            Neighbors = maps:get(Pt, Graph, []),
            MaxWeight = lists:foldl(
                fun(Nx, AccMax) ->
                    case maps:is_key(Nx, NewSeen) of
                        true ->
                            AccMax;
                        false ->
                            Weight = maps:get(Nx, maps:get(Pt, Graph, #{}), 0),
                            MaxOfNx = dfs(Graph, Nx, End, NewSeen, Max),
                            max(AccMax, MaxOfNx + Weight)
                    end
                end,
                -infinity,
                Neighbors
            ),
            MaxWeight
    end.

main() ->
    {ok, Input} = file:read_file("./input.txt"),

    res1 = long_walk(Input),
    io:format("~p~n", [res1]).

% res2 = long_walk(Input),
    % io:format("~p~n", [res2]).
