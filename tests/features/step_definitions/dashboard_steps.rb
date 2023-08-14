# frozen_string_literal: true

Given('I have the following stock transactions') do |table|
  # group by ticker symbol
  by_ticker ||= Hash.new { |h, k| h[k] = [] }
  table.symbolic_hashes.each do |row|
    by_ticker[row[:ticker]] << row
  end

  # INK: we already introduced an aggregate and the concept of Positions.
  # But nothing in te feature talks about that. Should we not just create events
  # and pass them into a Dashboard which is then decorated by a dashboard?
  @positions ||= []
  by_ticker.each do |ticker, rows|
    currency = rows.first[:currency]
    position = Position.new(ticker: ticker, currency: currency)
    rows.each { |row| position.add_transaction(amount: row[:amount].to_i, price: row[:price].to_i) }
    @positions << position
  end
end

When('I check my dashboard') do
  total_buying_price = @positions.reduce(0) { |sum, position| sum + position.total_buying_price }
  @output = DashboardView.new(total_buying_price: total_buying_price, currency: 'USD')
end

Then('I should see {string}') do |price|
  assert_includes(@output.to_s, price)
end
