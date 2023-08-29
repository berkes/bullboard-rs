Feature: Journal

  So that I can see my portfolio activities
  As a user
  I want to add entries to my journal
  And I want to see my journal entries

  Background:
    Given a database file to store events

  Scenario: Add a journal entry
    Given I have an empty journal
    When I add a journal entry
    Then I should see the entry in my journal

  Scenario: Save journal entries
    Given I have an empty journal
    When I add a journal entry
    And I restart the application
    Then I should see the entry in my journal

  Scenario: See multiple journal entries
    Given I have the following stock transactions
      | Ticker  | Currency | Amount  | Price |
      | AAPL    | USD      | 1       | 60    |
      | AAPL    | USD      | 1       | 90    |
      | TSLA    | USD      | 1       | 80    |
      | ESTC    | USD      | 3       | 20    |
    When "ESTC" pays "0.62 USD" dividend per share on "17-11-2021"
    When I check my journal
    Then I should see the following text
      """
      My Journal
        Date      Type      Ticker    Amount      Price        Total 
                Buy         AAPL           1    60.00 USD    60.00 USD 
                Buy         AAPL           1    90.00 USD    90.00 USD 
                Buy         TSLA           1    80.00 USD    80.00 USD 
                Buy         ESTC           3    20.00 USD    60.00 USD 
                Dividend    ESTC           1     0.62 USD     0.62 USD 
      """
