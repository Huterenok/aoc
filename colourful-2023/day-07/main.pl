#!/usr/bin/env perl

# perl ./main.pl

use strict;
use warnings;

sub findTotalWinnings {
  my $input = shift;

  my ($handsRef, $bidsRef) = retrieveHandsAndBids($input);
  my $withJokers = 0;

  my @sortedHands = sortHands($handsRef, $bidsRef, $withJokers);

  my $res = 0;
  for(my $i = 0; $i <= $#sortedHands; $i++) {
    $res += ($i + 1) * $sortedHands[$i]->{bid};
  }

  return $res;
}

sub findTotalJokerWinnings {
  my $input = shift;

  my ($handsRef, $bidsRef) = retrieveHandsAndBids($input);
  my $withJokers = 1;

  my @sortedHands = sortHands($handsRef, $bidsRef, $withJokers);

  my $res = 0;
  for(my $i = 0; $i <= $#sortedHands; $i++) {
    $res += ($i + 1) * $sortedHands[$i]->{bid};
  }

  return $res;
}

sub retrieveHandsAndBids {
  my $input = shift;
  my @lines = split "\n", $input;

  my @hands;
  my @bids;

  foreach my $line (@lines) {
    my ($hand, $bid) = split ' ', $line;
    push @hands, $hand;
    push @bids, $bid;
  }

  return (\@hands, \@bids);
}

sub sortHands {
  my ($handsRef, $bidsRef, $withJokers) = @_;

  my @tuples;
  for(my $i = 0; $i < @$handsRef; $i++) {
    push @tuples, {hand => $handsRef->[$i], bid => $bidsRef-> [$i]};
  }

  @tuples = sort {
    my $strengthCmp = findHandStrength($a->{hand}, $withJokers) <=> findHandStrength($b->{hand}, $withJokers);
    return $strengthCmp if $strengthCmp != 0;
    my $mismatchCompare = compareFirstHandsMismatch($a->{hand}, $b->{hand}, $withJokers);
    return $mismatchCompare;
  } @tuples;

  return @tuples;
}

sub findHandStrength {
  my ($hand, $withJokers)= @_;

  my $jokers = 0;
  my %letterHashes;
  foreach my $letter (split "", $hand) {
    if($withJokers && $letter eq 'J') {
      $jokers += 1;
      next;
    }
    $letterHashes{$letter}++;
  }

  my $max = 0;
  my $threes = 0;
  my $pair = 0;

  foreach my $value (values %letterHashes) {
    $max = $value + $jokers if $value + $jokers > $max;

    if ($value + $jokers == 3) {
      $threes = 1;
    } elsif ($value + $jokers == 2) {
      $pair += 1;
    }
  }

  return 6 if $max == 5;
  return 5 if $max == 4;
  return 4 if $threes && $pair;
  return 3 if $threes;
  return 2 if $pair == 2;
  return 1 if $pair == 1;
  return 0;
}

sub compareFirstHandsMismatch {
  my ($hand1, $hand2, $withJokers) = @_;

  for (my $i = 0; $i < length($hand1); $i++) {
    my $val1 = convertCharToNum(substr($hand1, $i, 1), $withJokers);
    my $val2 = convertCharToNum(substr($hand2, $i, 1), $withJokers);
    
    if ($val1 ne $val2) {
      return $val1 <=> $val2;
    }
  }

  return -1;
}

sub convertCharToNum {
  my ($card, $with_jokers) = @_;

  my $val;
  if ($card eq 'A') {
    $val = 13;
  } elsif ($card eq 'K') {
    $val = 12;
  } elsif ($card eq 'Q') {
    $val = 11;
  } elsif ($card eq 'J') {
    $val = $with_jokers ? 1 : 10;
  } elsif ($card eq 'T') {
    $val = $with_jokers ? 10 : 9;
  } else {
    $val = ord($card) - ord('0') - ($with_jokers ? 0 : 1);
  }

  return $val;
}

open(our $file, "<", "example_input.txt") or die "RETARDED!!!";
my $input = do { local $/; <$file> };

our $res1 = findTotalWinnings($input);
our $res2 = findTotalJokerWinnings($input);

print $res1."\n";
print $res2."\n";

close $file or die "RETARDED";