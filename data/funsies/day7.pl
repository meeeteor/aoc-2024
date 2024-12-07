:- use_module(library(dcg/basics), [whites//0]).
:- dynamic entry/2.

main :-
    retractall(entry(_, _)),  % Clear existing entries
    open('../inputs/07.txt', read, Stream),
	read_lines(Stream, Lines),
    close(Stream),
	maplist(parse_line, Lines),
	tally_score.

% EQUATION LOGIC:
valid_equation(Result, Numbers) :-
	reverse(Numbers, Reversed),
	generate_equations(Reversed, Result).

generate_equations([X], X).
generate_equations([H|T], Result) :-
	generate_equations(T, SubResult),
	(	Result is H + SubResult
	;	Result is H * SubResult
	;	concatenate(H, SubResult, Result)	% Add this one for part 2
	).

concatenate(H, SubResult, Result) :-
	number_codes(H, HCodes),
	number_codes(SubResult, SubCodes),
	append(SubCodes, HCodes, CombinedCodes),
	atom_codes(CombinedAtom, CombinedCodes),
	atom_number(CombinedAtom, Result).


tally_score :- 
	findall(X, entry(X, _), Xs),
	sum_list(Xs, Total),
	format('Total Score: ~w~n', [Total]).


% READING AND PARSING LOGIC:
read_lines(Stream, []) :-
    at_end_of_stream(Stream).
read_lines(Stream, [Line|Rest]) :-
    \+ at_end_of_stream(Stream),
    read_line_to_codes(Stream, Codes),
    atom_codes(Line, Codes),
    read_lines(Stream, Rest).

parse_line(Line) :-
    atom_codes(Line, Codes),
    phrase(line(X, Numbers), Codes),
	(	valid_equation(X, Numbers)
	->	assertz(entry(X, Numbers))
	;	true
	),
    format('Parsed: ~w: ~w~n', [X, Numbers]).

line(X, Numbers) -->
    parse_integer(X),
    ":",
    whites,
    numbers(Numbers).

numbers([N|Ns]) -->
    parse_integer(N),
    (   whites, numbers(Ns)
    ;   { Ns = [] }
    ).

parse_integer(I) -->
    digit(D0),
    digits(D),
    { number_codes(I, [D0|D]) }.

digits([D|T]) -->
    digit(D), !,
    digits(T).
digits([]) -->
    [].

digit(D) -->
    [D],
    { code_type(D, digit) }.