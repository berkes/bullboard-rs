Feature: Dashboard

  So that I can see the most important figures at a glance
  As a user
  I want to see important numbers

  Scenario: Portfolio Table
    Given I have the following stock transactions
      | Ticker  | Currency | Amount  | Price |
      | AAPL    | USD      | 1       | 60    |
      | AAPL    | USD      | 1       | 90    |
      | TSLA    | USD      | 1       | 80    |
      | ESTC    | USD      | 3       | 20    |
    # TODO: should price-obtained be emitted on stocks bought? Or shold we explicitly obtain the price?
    When the prices change to the following values on "17-11-2021"
      | Ticker  | Price |
      | AAPL    | 71    |
      | ESTC    | 22    |
    When I check my dashboard
    Then I should see the following text
      # NOTE: This is a string literal, not a Gherkin table
      # NOTE: The rows are sorted by value
      """
      Dashboard
      Amount of positions: 3
      Total Buying Price:  290.00 USD
      Total Value:         208.00 USD
      Total dividend:      0.00 USD

        Ticker    Amount      Value 
        AAPL           2    142.00 USD 
        ESTC           3     66.00 USD 
        TSLA           1     ??.?? ??? 
      """
      # TODO: add columns: name, unrealized P/L, realized P/L, dividend, Total P/L
      # TODO: add percentages of gains/losses for each position sinice last price check

  Scenario: Dividend
    Given I have the following stock transactions
      | Ticker  | Currency | Amount  | Price |
      | MSFT    | USD      | 5       | 60    |
    When "MSFT" pays "0.62 USD" dividend per share on "17-11-2021"
    # So that we test that purchases after ex-divident date don't count
    When I have the following stock transactions
      | Ticker  | Currency | Amount  | Price |
      | MSFT    | USD      | 15      | 60    |
    When I check my dashboard
    Then I should see "Total dividend: 3.10 USD"

