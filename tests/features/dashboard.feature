Feature: Dashboard

  So that I can see the most important figures at a glance
  As a user
  I want to see important numbers

  Scenario: Total Buy Price
    Given I have the following stock transactions
      | Ticker  | Currency | Amount | Price |
      | AAPL    | USD      | 1      | 60    |
      | AAPL    | USD      | 1      | 80    |
      | ESTC    | USD      | 3      | 20    |
    When I check my dashboard
    Then I should see "Total Buying Price: 200 USD"

  Scenario: Total Value over Time
    Given I have the following stock transactions
      | Ticker  | Currency | Amount | Price |
      | AAPL    | USD      | 1      | 60    |
      | ESTC    | USD      | 3      | 20    |
    When the prices change to the following values on "17-11-2021"
      | Ticker  | Price |
      | AAPL    | 71    |
      | ESTC    | 22    |
    When the prices change to the following values on "24-11-2021"
      | Ticker  | Price |
      | AAPL    | 80    |
      | ESTC    | 19    |
    When I check my dashboard
    Then I should see "Total Value: 137 USD"

  Scenario: Total Positions
    Given I have the following stock transactions
      | Ticker  | Currency | Amount  | Price |
      | AAPL    | USD      | 1       | 60    |
      | AAPL    | USD      | 1       | 90    |
      | TSLA    | USD      | 1       | 80    |
      | ESTC    | USD      | 3       | 20    |
    When I check my dashboard
    Then I should see "Amount of positions: 3"

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

