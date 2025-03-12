BIND = '127.0.0.2'
PORT = 8188

help:
	@cat Makefile

run:
	@cargo run -r -- -b $(BIND) -p $(PORT)

test:
	@echo
	curl -w '\nStatusCode: %{http_code}\n' http://$(BIND):$(PORT)/tasks
	@echo
	curl -w '\nStatusCode: %{http_code}\n' http://$(BIND):$(PORT)/task/1
	@echo
	curl -w '\nStatusCode: %{http_code}\n' http://$(BIND):$(PORT)/task/3
	@echo
	curl -w '\nStatusCode: %{http_code}\n' http://$(BIND):$(PORT)/task/3 -X DELETE
	@echo
	curl -w '\nStatusCode: %{http_code}\n' http://$(BIND):$(PORT)/ -X POST -d 'newItem'
	@echo
	curl -w '\nStatusCode: %{http_code}\n' http://$(BIND):$(PORT)/task/1 -X DELETE
	@echo
	curl -w '\nStatusCode: %{http_code}\n' http://$(BIND):$(PORT)/tasks

# # # # # # # #
pull:
	@git pull

savetogit: git.pushall
git.pushall: git.commitall
	@git push
git.commitall: git.addall
	@if [ -n "$(shell git status -s)" ] ; then git commit -m 'saving'; else echo '--- nothing to commit'; fi
git.addall:
	@git add .

# # # # # # # #
clean:
	@cargo clean
