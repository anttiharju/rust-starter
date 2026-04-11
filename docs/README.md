# rust-starter

`rust-starter` is a template for my Rust projects to make it easier to start new ones.

How to use (only works for Antti):

1. Use template. Do not choose a repository name that ends with `-starter`
2. About -> Gear icon -> Edit repository details and
   - Add Description
   - Fill in Website
3. Add `ANTTIHARJU_BOT_ID` and `ANTTIHARJU_BOT_PRIVATE_KEY` in Settings -> Secrets and Variables -> Actions
4. When opening the first pull request, go Labels -> Edit labels -> New label and add:
   - `major-release` (click on hex color to reveal defaults, leftmost / top row dark red)
   - `minor-release` (2nd leftmost / top row orange)
   - `patch-release` (3rd leftmost / top row yellow)
5. Branch off from main, find-and-replace `rust-starter` with your new repository name.
6. Go to repository Settings and
   - Under Releases check Enable release immutability
   - Disable unnecessary features like Wikis and Projects
   - Allow auto-merge
7. Go to repository Settings -> Rules -> Rulesets -> Add new branch ruleset and:
   - name it 'default branch'
   - Set enforcement status to active
   - Add bypass for Repository admin and anttiharju App
   - For Target branches Add target Include default branch
   - Under Branch rules:
     - Require a pull request before merging
     - Require status checks to pass
       - Add checks -> Plan / Validate
   - Click Create
8. Go to repository Settings -> Pages
   - Source -> GitHub Actions
   - Check Enforce HTTPS
