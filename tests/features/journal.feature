Feature: Journal

  So that I can see my portfolio activities
  As a user
  I want to add entries to my journal
  And I want to see my journal entries

  Background:
    Given a database file to store events

  Scenario: See multiple journal entries
    Given I have the following stock transactions
      | Ticker | Currency | Amount | Price | Date      |
      | AAPL   | USD      | 1      | 60    | 2021-10-1 |
      | AAPL   | USD      | 1      | 90    | 2021-11-1 |
      | TSLA   | USD      | 1      | 80    | 2021-12-1 |
      | ESTC   | USD      | 3      | 20    | 2022-1-1  |
    When "ESTC" pays "0.62 USD" dividend per share on "2021-11-17"
    When I check my journal
    Then I should see the following text
      """
      My Journal
           Date         Type      Ticker    Amount      Price        Total 
        2021-10-01    Buy         AAPL           1    60.00 USD    60.00 USD 
        2021-11-01    Buy         AAPL           1    90.00 USD    90.00 USD 
        2021-11-17    Dividend    ESTC           1     0.62 USD     0.62 USD 
        2021-12-01    Buy         TSLA           1    80.00 USD    80.00 USD 
        2022-01-01    Buy         ESTC           3    20.00 USD    60.00 USD 
      """
